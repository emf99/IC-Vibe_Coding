use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use std::cell::RefCell;

mod config;
use config::Config;

thread_local! {
    static COUNTER: RefCell<u64> = const { RefCell::new(0) };
}

#[derive(CandidType, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(CandidType, Deserialize)]
pub struct SupabaseResponse {
    pub data: Option<String>,
    pub error: Option<String>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct QueryParseResult {
    pub table: String,
    pub query: String,
    pub error: Option<String>,
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
fn increment() -> u64 {
    COUNTER.with(|counter| {
        let mut count = counter.borrow_mut();
        *count += 1;
        *count
    })
}

#[ic_cdk::query]
fn get_count() -> u64 {
    COUNTER.with(|counter| *counter.borrow())
}

#[ic_cdk::update]
fn set_count(value: u64) -> u64 {
    COUNTER.with(|counter| {
        *counter.borrow_mut() = value;
        value
    })
}

#[ic_cdk::update]
async fn fetch_from_supabase_no_encoding(
    table: String,
    query: String,
) -> Result<SupabaseResponse, String> {
    let supabase_url = Config::supabase_url();
    let supabase_key = Config::supabase_anon_key();

    let url = if query.is_empty() {
        format!("{}/rest/v1/{}", supabase_url, table)
    } else {
        // Don't URL encode - try direct concatenation
        let final_url = format!("{}/rest/v1/{}?{}", supabase_url, table, query);
        ic_cdk::println!("Final URL (no encoding): {}", final_url);
        final_url
    };

    // ... rest of the function same as original fetch_from_supabase
    let request_headers = vec![
        HttpHeader {
            name: "apikey".to_string(),
            value: supabase_key.to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", supabase_key),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Accept".to_string(),
            value: "application/json".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(8192),
        transform: Some(TransformContext::from_name(
            "transform".to_string(),
            serde_json::to_vec(&()).unwrap(),
        )),
        headers: request_headers,
    };

    match http_request(request, 50_000_000_000).await {
        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .map_err(|_| "Failed to parse response body as UTF-8".to_string())?;

            let status_code: u32 = response.status.0.to_string().parse().unwrap_or(500);
            ic_cdk::println!("Response status: {}", status_code);
            ic_cdk::println!("Response body: {}", str_body);

            if (200..300).contains(&status_code) {
                Ok(SupabaseResponse {
                    data: Some(str_body),
                    error: None,
                })
            } else {
                Ok(SupabaseResponse {
                    data: None,
                    error: Some(format!("HTTP {} - {}", status_code, str_body)),
                })
            }
        }
        Err((r, m)) => {
            let message = format!("HTTP request failed with code {:?}: {}", r, m);
            Ok(SupabaseResponse {
                data: None,
                error: Some(message),
            })
        }
    }
}

#[ic_cdk::update]
async fn chat(messages: Vec<ChatMessage>) -> String {
    format!("Received {} messages", messages.len())
}

#[ic_cdk::update]
async fn debug_parse_query(user_query: String) -> Result<QueryParseResult, String> {
    ic_cdk::println!("=== DEBUG PARSE QUERY ===");
    ic_cdk::println!("Input query: {}", user_query);

    // Test both methods
    let llm_result = parse_natural_language_query_with_llm(user_query.clone()).await;
    let fallback_result = parse_natural_language_query_fallback(user_query.clone()).await;

    ic_cdk::println!("LLM result: {:?}", llm_result);
    ic_cdk::println!("Fallback result: {:?}", fallback_result);

    // Return the fallback result for now since LLM isn't working properly
    llm_result
}

// src/backend/src/lib.rs

// Update the LLM types to match the canister interface
#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub struct ChatMessageV0 {
    pub content: String,
    pub role: ChatRoleV0,
}

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub enum ChatRoleV0 {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

#[derive(candid::CandidType, serde::Deserialize, Debug, Clone)]
pub struct ChatRequestV0 {
    pub model: String,
    pub messages: Vec<ChatMessageV0>,
}

// src/backend/src/lib.rs

#[ic_cdk::update]
async fn parse_natural_language_query_with_llm(
    user_query: String,
) -> Result<QueryParseResult, String> {
    let llm_canister_id = Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai")
        .map_err(|_| "Invalid LLM canister ID".to_string())?;

    ic_cdk::println!(
        "Calling LLM service parse_natural_language_to_sql with query: {}",
        user_query
    );

    // Call the LLM service using parse_natural_language_to_sql method
    let llm_response: Result<(Result<QueryParseResult, String>,), _> = ic_cdk::call(
        llm_canister_id,
        "parse_natural_language_to_sql",
        (user_query.clone(),),
    )
    .await;

    match llm_response {
        Ok((result,)) => match result {
            Ok(query_result) => {
                ic_cdk::println!(
                    "Successfully parsed via LLM service: table={}, query={}",
                    query_result.table,
                    query_result.query
                );
                Ok(query_result)
            }
            Err(err) => {
                ic_cdk::println!("LLM service returned error: {}, using fallback", err);
                parse_natural_language_query_fallback(user_query).await
            }
        },
        Err(e) => {
            ic_cdk::println!("LLM service call failed: {:?}", e);
            // Fallback to manual parsing
            parse_natural_language_query_fallback(user_query).await
        }
    }
}
// Fallback parsing function (improved simple approach)
#[ic_cdk::update]
async fn parse_natural_language_query_fallback(
    user_query: String,
) -> Result<QueryParseResult, String> {
    let user_query_lower = user_query.to_lowercase();

    // Validate if this looks like a database query
    let database_keywords = [
        "todo",
        "task",
        "user",
        "post",
        "show",
        "get",
        "find",
        "list",
        "all",
        "completed",
        "done",
        "incomplete",
        "id",
    ];

    let has_database_keywords = database_keywords
        .iter()
        .any(|&keyword| user_query_lower.contains(keyword));

    if !has_database_keywords {
        return Ok(QueryParseResult {
            table: "".to_string(),
            query: "".to_string(),
            error: Some(format!(
                "Unable to parse '{}' as a database query. Please use keywords like 'show', 'get', 'todos', 'users', etc.",
                user_query
            )),
        });
    }

    // Extract table name
    let table = if user_query_lower.contains("todos") || user_query_lower.contains("todo") {
        "todos".to_string()
    } else if user_query_lower.contains("users") || user_query_lower.contains("user") {
        "users".to_string()
    } else if user_query_lower.contains("posts") || user_query_lower.contains("post") {
        "posts".to_string()
    } else {
        return Ok(QueryParseResult {
            table: "".to_string(),
            query: "".to_string(),
            error: Some(
                "Could not identify table from query. Please mention 'todos', 'users', or 'posts'"
                    .to_string(),
            ),
        });
    };

    // Extract query type and build Supabase query - Use correct column names
    let query = if user_query_lower.contains("all") || user_query_lower.contains("everything") {
        "select=*".to_string()
    } else if user_query_lower.contains("completed") || user_query_lower.contains("done") {
        if user_query_lower.contains("not")
            || user_query_lower.contains("incomplete")
            || user_query_lower.contains("false")
            || user_query_lower.contains("unfinished")
        {
            "select=*&is_done=eq.false".to_string()
        } else {
            "select=*&is_done=eq.true".to_string()
        }
    } else if user_query_lower.contains("id") {
        // Try to extract ID number
        let words: Vec<&str> = user_query_lower.split_whitespace().collect();
        if let Some(id_pos) = words.iter().position(|&w| w == "id") {
            if let Some(id_value) = words.get(id_pos + 1) {
                if id_value.parse::<i32>().is_ok() {
                    format!("select=*&id=eq.{}", id_value)
                } else {
                    "select=*".to_string()
                }
            } else {
                "select=*".to_string()
            }
        } else {
            "select=*".to_string()
        }
    } else if user_query_lower.contains("show")
        || user_query_lower.contains("get")
        || user_query_lower.contains("list")
    {
        // Allow basic "show" commands even without specific filters
        "select=*".to_string()
    } else {
        return Ok(QueryParseResult {
            table: "".to_string(),
            query: "".to_string(),
            error: Some(format!(
                "Could not understand what to do with '{}'. Try 'show all todos', 'get completed tasks', etc.",
                user_query
            )),
        });
    };

    Ok(QueryParseResult {
        table,
        query,
        error: None,
    })
}

// Nowa funkcja używająca naszego własnego kanister LLM service
#[ic_cdk::update]
async fn parse_with_llm_service(user_query: String) -> Result<QueryParseResult, String> {
    ic_cdk::println!("Using LLM service to parse query: {}", user_query);

    // ID naszego kanister LLM service
    let llm_service_canister_id = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai")
        .map_err(|_| "Invalid LLM service canister ID".to_string())?;

    ic_cdk::println!("Calling LLM service canister: {}", llm_service_canister_id);

    // Wywołaj nasz kanister LLM service
    let llm_response: Result<(Result<QueryParseResult, String>,), _> = ic_cdk::call(
        llm_service_canister_id,
        "parse_natural_language_to_sql",
        (user_query.clone(),),
    )
    .await;

    match llm_response {
        Ok((result,)) => match result {
            Ok(parsed_result) => {
                ic_cdk::println!(
                    "LLM service successfully parsed: table={}, query={}",
                    parsed_result.table,
                    parsed_result.query
                );
                Ok(parsed_result)
            }
            Err(error) => {
                ic_cdk::println!("LLM service returned error: {}", error);
                parse_enhanced_fallback(user_query).await
            }
        },
        Err(e) => {
            ic_cdk::println!("Failed to call LLM service canister: {:?}", e);
            parse_enhanced_fallback(user_query).await
        }
    }
}

// Ulepszona wersja fallback parsera
#[ic_cdk::update]
async fn parse_enhanced_fallback(user_query: String) -> Result<QueryParseResult, String> {
    let query_lower = user_query.to_lowercase();
    ic_cdk::println!("Parsing with enhanced fallback: {}", query_lower);

    // First validate if this looks like a meaningful database query
    let database_keywords = [
        "todo",
        "task",
        "user",
        "post",
        "show",
        "get",
        "find",
        "list",
        "all",
        "completed",
        "done",
        "incomplete",
        "pending",
        "due",
        "date",
        "title",
        "id",
    ];

    let has_database_keywords = database_keywords
        .iter()
        .any(|&keyword| query_lower.contains(keyword));

    if !has_database_keywords {
        return Ok(QueryParseResult {
            table: "".to_string(),
            query: "".to_string(),
            error: Some(format!(
                "Unable to parse '{}' as a database query. Please use words like 'show todos', 'get users', 'find completed tasks', etc.",
                user_query
            )),
        });
    }

    // Określ tabelę
    let table = if query_lower.contains("todo") || query_lower.contains("task") {
        "todos"
    } else if query_lower.contains("user") {
        "users"
    } else if query_lower.contains("post") {
        "posts"
    } else {
        // Only default to todos if there are other valid query keywords
        if query_lower.contains("show")
            || query_lower.contains("get")
            || query_lower.contains("find")
            || query_lower.contains("all")
            || query_lower.contains("completed")
            || query_lower.contains("done")
        {
            "todos"
        } else {
            return Ok(QueryParseResult {
                table: "".to_string(),
                query: "".to_string(),
                error: Some(format!(
                    "Could not determine table from query '{}'. Please specify 'todos', 'users', or 'posts'.",
                    user_query
                )),
            });
        }
    };

    // Zbuduj query
    let mut query_parts = vec!["select=*".to_string()];
    let mut has_filters = false;

    // Filtry dla todos
    if table == "todos" {
        if query_lower.contains("completed") || query_lower.contains("done") {
            query_parts.push("is_done=eq.true".to_string());
            has_filters = true;
        } else if query_lower.contains("incomplete")
            || query_lower.contains("not done")
            || query_lower.contains("pending")
        {
            query_parts.push("is_done=eq.false".to_string());
            has_filters = true;
        }

        if query_lower.contains("due date") && query_lower.contains("not null") {
            query_parts.push("due_date=not.is.null".to_string());
            has_filters = true;
        } else if query_lower.contains("due date") && query_lower.contains("null") {
            query_parts.push("due_date=is.null".to_string());
            has_filters = true;
        }

        // Wyszukiwanie tekstowe w tytule
        if let Some(search_term) = extract_search_term(&query_lower) {
            let search_query = format!("title=ilike.*{}*", search_term);
            query_parts.push(search_query);
            has_filters = true;
        }
    }

    // If no specific filters were found but we have valid keywords, allow basic "all" queries
    if !has_filters
        && !query_lower.contains("all")
        && !query_lower.contains("everything")
        && !query_lower.contains("show")
        && !query_lower.contains("get")
        && !query_lower.contains("list")
    {
        return Ok(QueryParseResult {
            table: "".to_string(),
            query: "".to_string(),
            error: Some(format!(
                "Query '{}' doesn't specify what to retrieve. Try 'show all todos', 'get completed tasks', etc.",
                user_query
            )),
        });
    }

    let final_query = query_parts.join("&");

    Ok(QueryParseResult {
        table: table.to_string(),
        query: final_query,
        error: None,
    })
}

// Funkcja pomocnicza do wyciągania terminu wyszukiwania
fn extract_search_term(query: &str) -> Option<String> {
    // Proste parsowanie terminu w cudzysłowach lub po "with", "containing", etc.
    if let Some(start) = query.find('"') {
        if let Some(end) = query[start + 1..].find('"') {
            return Some(query[start + 1..start + 1 + end].to_string());
        }
    }

    // Szukaj po "with" lub "containing"
    for keyword in &["with ", "containing ", "about "] {
        if let Some(pos) = query.find(keyword) {
            let remainder = &query[pos + keyword.len()..];
            if let Some(word_end) = remainder.find(' ') {
                return Some(remainder[..word_end].to_string());
            } else {
                return Some(remainder.to_string());
            }
        }
    }

    None
}

// ...existing code...

#[ic_cdk::update]
async fn query_supabase_with_natural_language(
    user_query: String,
) -> Result<SupabaseResponse, String> {
    ic_cdk::println!("=== MAIN QUERY FUNCTION DEBUG ===");
    ic_cdk::println!("User query: {}", user_query);

    // Use fallback parsing directly for now
    let parse_result = parse_natural_language_query_with_llm(user_query).await?;

    ic_cdk::println!(
        "Parse result: table={}, query={}",
        parse_result.table,
        parse_result.query
    );

    if let Some(error) = parse_result.error {
        return Ok(SupabaseResponse {
            data: None,
            error: Some(error),
        });
    }

    // Use the non-encoding version that we know works
    fetch_from_supabase_no_encoding(parse_result.table, parse_result.query).await
}

// Update the main fetch function to not use URL encoding
#[ic_cdk::update]
async fn fetch_from_supabase(table: String, query: String) -> Result<SupabaseResponse, String> {
    let supabase_url = Config::supabase_url();
    let supabase_key = Config::supabase_anon_key();

    ic_cdk::println!(
        "Fetching from Supabase - Table: {}, Query: {}",
        table,
        query
    );

    let url = if query.is_empty() {
        format!("{}/rest/v1/{}", supabase_url, table)
    } else {
        // Don't URL encode - use direct concatenation since it works
        format!("{}/rest/v1/{}?{}", supabase_url, table, query)
    };

    ic_cdk::println!("Final URL: {}", url);

    let request_headers = vec![
        HttpHeader {
            name: "apikey".to_string(),
            value: supabase_key.to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", supabase_key),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Accept".to_string(),
            value: "application/json".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(8192),
        transform: Some(TransformContext::from_name(
            "transform".to_string(),
            serde_json::to_vec(&()).unwrap(),
        )),
        headers: request_headers,
    };

    match http_request(request, 50_000_000_000).await {
        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .map_err(|_| "Failed to parse response body as UTF-8".to_string())?;

            let status_code: u32 = response.status.0.to_string().parse().unwrap_or(500);
            ic_cdk::println!("Response status: {}, body: {}", status_code, str_body);

            if (200..300).contains(&status_code) {
                Ok(SupabaseResponse {
                    data: Some(str_body),
                    error: None,
                })
            } else {
                Ok(SupabaseResponse {
                    data: None,
                    error: Some(format!("HTTP {} - {}", status_code, str_body)),
                })
            }
        }
        Err((r, m)) => {
            let message = format!("HTTP request failed with code {:?}: {}", r, m);
            Ok(SupabaseResponse {
                data: None,
                error: Some(message),
            })
        }
    }
}

#[ic_cdk::update]
async fn insert_to_supabase(table: String, data: String) -> Result<SupabaseResponse, String> {
    let supabase_url = Config::supabase_url();
    let supabase_key = Config::supabase_anon_key();

    let url = format!("{}/rest/v1/{}", supabase_url, table);

    let request_headers = vec![
        HttpHeader {
            name: "apikey".to_string(),
            value: supabase_key.to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", supabase_key),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Accept".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Prefer".to_string(),
            value: "return=representation".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        method: HttpMethod::POST,
        body: Some(data.into_bytes()),
        max_response_bytes: Some(8192),
        transform: Some(TransformContext::from_name(
            "transform".to_string(),
            serde_json::to_vec(&()).unwrap(),
        )),
        headers: request_headers,
    };

    match http_request(request, 50_000_000_000).await {
        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .map_err(|_| "Failed to parse response body as UTF-8".to_string())?;

            let status_code: u32 = response.status.0.to_string().parse().unwrap_or(500);
            if (200..300).contains(&status_code) {
                Ok(SupabaseResponse {
                    data: Some(str_body),
                    error: None,
                })
            } else {
                Ok(SupabaseResponse {
                    data: None,
                    error: Some(format!("HTTP {} - {}", status_code, str_body)),
                })
            }
        }
        Err((r, m)) => {
            let message = format!("HTTP request failed with code {:?}: {}", r, m);
            Ok(SupabaseResponse {
                data: None,
                error: Some(message),
            })
        }
    }
}

#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let mut sanitized_headers = Vec::new();
    for header in raw.response.headers.iter() {
        if header.name.to_lowercase().starts_with("x-")
            || header.name.to_lowercase() == "content-type"
            || header.name.to_lowercase() == "accept"
        {
            sanitized_headers.push(header.clone());
        }
    }

    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers: sanitized_headers,
    }
}
#[ic_cdk::update]
async fn create_test_todos() -> Result<SupabaseResponse, String> {
    // Fix: Include user_id in the test data to satisfy the NOT NULL constraint
    let test_todos = r#"[
        {"title": "Buy groceries", "is_done": false, "user_id": "123e4567-e89b-12d3-a456-426614174000"},
        {"title": "Walk the dog", "is_done": true, "user_id": "123e4567-e89b-12d3-a456-426614174000"}, 
        {"title": "Finish project", "is_done": false, "user_id": "123e4567-e89b-12d3-a456-426614174000"},
        {"title": "Read book", "is_done": true, "user_id": "123e4567-e89b-12d3-a456-426614174000"},
        {"title": "Learn Rust", "is_done": true, "user_id": "123e4567-e89b-12d3-a456-426614174000"},
        {"title": "Build IC app", "is_done": false, "user_id": "123e4567-e89b-12d3-a456-426614174000"}
    ]"#;

    ic_cdk::println!("Creating test todos with data: {}", test_todos);
    insert_to_supabase("todos".to_string(), test_todos.to_string()).await
}

// Add missing prompt function with robust error handling
#[ic_cdk::update]
async fn prompt(user_prompt: String) -> String {
    ic_cdk::println!("Received prompt: {}", user_prompt);

    // Check if this is a database query first
    let prompt_lower = user_prompt.to_lowercase();
    if prompt_lower.contains("todo")
        || prompt_lower.contains("user")
        || prompt_lower.contains("post")
        || prompt_lower.contains("show")
        || prompt_lower.contains("get")
        || prompt_lower.contains("find")
        || prompt_lower.contains("all")
        || prompt_lower.contains("select")
    {
        ic_cdk::println!("Detected database query, processing with natural language parser");

        // Use the existing natural language processing for database queries
        match query_supabase_with_natural_language(user_prompt.clone()).await {
            Ok(response) => {
                if let Some(data) = response.data {
                    format!("Database query executed successfully. Results:\n{}", data)
                } else if let Some(error) = response.error {
                    format!("Database query failed: {}", error)
                } else {
                    "Database query executed but no data returned".to_string()
                }
            }
            Err(error) => {
                format!("Error processing database query: {}", error)
            }
        }
    } else {
        // For general LLM queries, handle the timeout issue gracefully
        ic_cdk::println!("Detected general LLM query, attempting to call LLM canister");

        let llm_canister_id_result = Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai");

        match llm_canister_id_result {
            Ok(llm_canister_id) => {
                // Define the chat message types matching LLM canister exactly
                #[derive(CandidType)]
                struct ChatRequestV0 {
                    model: String,
                    messages: Vec<ChatMessageV0>,
                }

                #[derive(CandidType)]
                struct ChatMessageV0 {
                    content: String,
                    role: ChatRoleV0,
                }

                #[derive(CandidType)]
                enum ChatRoleV0 {
                    User,
                    Assistant,
                    System,
                }

                let chat_request = ChatRequestV0 {
                    model: "llama3.1:8b".to_string(),
                    messages: vec![ChatMessageV0 {
                        content: user_prompt.clone(),
                        role: ChatRoleV0::User,
                    }],
                };

                ic_cdk::println!("Calling LLM canister with v0_chat method - this may take time for model loading");

                // Add a longer timeout and better error handling for model loading
                match ic_cdk::call::<(ChatRequestV0,), (String,)>(
                    llm_canister_id,
                    "v0_chat",
                    (chat_request,),
                )
                .await
                {
                    Ok((response,)) => {
                        ic_cdk::println!("LLM canister responded successfully");
                        response
                    }
                    Err(e) => {
                        ic_cdk::println!("LLM canister call failed: {:?}", e);

                        // Provide a helpful response when LLM fails
                        let error_msg = format!("{:?}", e);
                        if error_msg.contains("Timeout") || error_msg.contains("timeout") {
                            format!(
                                "I'm sorry, the AI model is taking longer than expected to respond (likely due to model loading time). \
                                This is common on the first request. Please try again in a moment, or try a database-related query like 'show all todos' which I can handle directly.\n\n\
                                Your original question was: \"{}\"", 
                                user_prompt
                            )
                        } else if error_msg.contains("Panicked") || error_msg.contains("trap") {
                            format!(
                                "I'm experiencing a technical issue with the AI model service. \
                                However, I can help you with database queries like 'show all todos', 'get completed tasks', etc.\n\n\
                                For general questions, please try again later when the AI service is more stable.\n\n\
                                Your original question was: \"{}\"", 
                                user_prompt
                            )
                        } else {
                            format!(
                                "I'm currently unable to process your request due to a technical issue: {}\n\n\
                                I can help you with database queries instead. Try asking 'show all todos' or 'get my tasks'.\n\n\
                                Your original question was: \"{}\"", 
                                error_msg, user_prompt
                            )
                        }
                    }
                }
            }
            Err(_) => {
                ic_cdk::println!("Invalid LLM canister ID");
                format!(
                    "I'm sorry, there's a configuration issue with the AI service. \
                    However, I can help you with database queries like 'show all todos', 'find completed tasks', etc.\n\n\
                    Your question was: \"{}\"", 
                    user_prompt
                )
            }
        }
    }
}

// Add a warm-up function to pre-load the LLM model
#[ic_cdk::update]
async fn warm_up_llm() -> String {
    ic_cdk::println!("Warming up LLM model - this will pre-load llama3.1:8b");

    let llm_canister_id_result = Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai");

    match llm_canister_id_result {
        Ok(llm_canister_id) => {
            #[derive(CandidType)]
            struct ChatRequestV0 {
                model: String,
                messages: Vec<ChatMessageV0>,
            }

            #[derive(CandidType)]
            struct ChatMessageV0 {
                content: String,
                role: ChatRoleV0,
            }

            #[derive(CandidType)]
            enum ChatRoleV0 {
                User,
                Assistant,
                System,
            }

            let chat_request = ChatRequestV0 {
                model: "llama3.1:8b".to_string(),
                messages: vec![ChatMessageV0 {
                    content: "Hello".to_string(), // Simple warm-up message
                    role: ChatRoleV0::User,
                }],
            };

            ic_cdk::println!("Sending warm-up request to LLM canister");

            match ic_cdk::call::<(ChatRequestV0,), (String,)>(
                llm_canister_id,
                "v0_chat",
                (chat_request,),
            )
            .await
            {
                Ok((response,)) => {
                    ic_cdk::println!("LLM warm-up successful");
                    format!("LLM model warmed up successfully. Response: {}", response)
                }
                Err(e) => {
                    ic_cdk::println!("LLM warm-up failed: {:?}", e);
                    format!("LLM warm-up failed: {:?}", e)
                }
            }
        }
        Err(_) => "Invalid LLM canister ID".to_string(),
    }
}

ic_cdk::export_candid!();
