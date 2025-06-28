use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<u64> = RefCell::new(0);
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
    let supabase_url = "https://tgsgxbmwhwcymfuodokl.supabase.co";
    let supabase_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InRnc2d4Ym13aHdjeW1mdW9kb2tsIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTA2NjY2ODQsImV4cCI6MjA2NjI0MjY4NH0.pkUJjiSEAzn3sG-C8iqdkyXPIWTHjypZ8HH31166uYM";

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

            if status_code >= 200 && status_code < 300 {
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
/*
#[ic_cdk::update]
async fn parse_natural_language_query_with_llm(
    user_query: String,
) -> Result<QueryParseResult, String> {
    // Get the LLM canister ID from environment or use hardcoded value
    let llm_canister_id = Principal::from_text("w36hm-eqaaa-aaaal-qr76a-cai")
        .map_err(|_| "Invalid LLM canister ID".to_string())?;

    // Create a detailed prompt for the LLM to parse the query
    let system_prompt = r#"You are a database query parser. Parse natural language queries into Supabase REST API format.

Available tables and columns:
- todos: id (integer), title (text), is_done (boolean), created_at (timestamp)
- users: id (integer), name (text), email (text), created_at (timestamp)
- posts: id (integer), title (text), content (text), user_id (integer), created_at (timestamp)

Convert natural language to Supabase query format:
- "select=*" for all columns
- "select=id,title" for specific columns
- "completed=eq.true" for boolean filters (note: use 'completed', not 'is_done')
- "due_date=not.is.null" for non-null date filters
- "due_date=is.null" for null date filters
- "id=eq.1" for exact matches
- "title=ilike.*search*" for text search
- "status=eq.pending" for status filters

Respond ONLY with JSON in this exact format:
{"table": "table_name", "query": "supabase_query_string", "error": null}

If you can't parse the query, respond with:
{"table": "", "query": "", "error": "explanation"}

Examples:
"get all todos" → {"table": "todos", "query": "select=*", "error": null}
"show completed todos" → {"table": "todos", "query": "select=*&is_done=eq.true", "error": null}
"find incomplete todos" → {"table": "todos", "query": "select=*&is_done=eq.false", "error": null}"#;

    let user_prompt = format!("Parse this query: {}", user_query);
    let full_prompt = format!("{}\n\n{}", system_prompt, user_prompt);

    // Try different method names that might exist on the LLM canister
    let llm_response = match ic_cdk::call::<(String,), (String,)>(
        llm_canister_id,
        "generate", // Try 'generate' first
        (full_prompt.clone(),),
    )
    .await
    {
        Ok(response) => response,
        Err(_) => {
            // If 'generate' fails, try 'chat'
            match ic_cdk::call::<(Vec<ChatMessage>,), (String,)>(
                llm_canister_id,
                "chat",
                (vec![ChatMessage {
                    role: "user".to_string(),
                    content: full_prompt.clone(),
                }],),
            )
            .await
            {
                Ok(response) => (response.0,),
                Err(_) => {
                    // If both fail, try 'complete'
                    match ic_cdk::call::<(String,), (String,)>(
                        llm_canister_id,
                        "complete",
                        (full_prompt,),
                    )
                    .await
                    {
                        Ok(response) => response,
                        Err(e) => {
                            // If all methods fail, fall back to simple parsing
                            ic_cdk::println!("LLM canister call failed: {:?}", e);
                            return parse_natural_language_query_fallback(user_query).await;
                        }
                    }
                }
            }
        }
    };

    // Parse the LLM response as JSON
    let response_text = llm_response.0;

    // Try to parse the JSON response
    match serde_json::from_str::<QueryParseResult>(&response_text) {
        Ok(result) => Ok(result),
        Err(_) => {
            // If JSON parsing fails, try to extract meaningful info from text response
            ic_cdk::println!("Failed to parse LLM JSON response: {}", response_text);
            parse_natural_language_query_fallback(user_query).await
        }
    }
} */
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
    let llm_canister_id = Principal::from_text("w36hm-eqaaa-aaaal-qr76a-cai")
        .map_err(|_| "Invalid LLM canister ID".to_string())?;

    // Create the system message with database schema
    let system_message = ChatMessageV0 {
        content: r#"You are a database query parser. Convert natural language queries into Supabase REST API format.

Database schema:
- todos: id (integer), title (text), description (text), is_done (boolean), due_date (timestamp), status (text), created_at (timestamp)
- users: id (integer), name (text), email (text), created_at (timestamp)  
- posts: id (integer), title (text), content (text), user_id (integer), created_at (timestamp)

Convert to Supabase PostgREST format:
- "select=*" for all columns
- "select=id,title" for specific columns  
- "is_done=eq.true" for boolean filters
- "due_date=not.is.null" for non-null filters
- "due_date=is.null" for null filters
- "title=ilike.*search*" for text search

Respond ONLY with JSON:
{"table": "table_name", "query": "supabase_query_string", "error": null}

Examples:
"get all todos" → {"table": "todos", "query": "select=*", "error": null}
"show completed todos" → {"table": "todos", "query": "select=*&is_done=eq.true", "error": null}
"show todos where due date is not null" → {"table": "todos", "query": "select=*&due_date=not.is.null", "error": null}
"find incomplete todos" → {"table": "todos", "query": "select=*&is_done=eq.false", "error": null}"#.to_string(),
        role: ChatRoleV0::System,
    };

    // Create the user message
    let user_message = ChatMessageV0 {
        content: format!("Parse this query: {}", user_query),
        role: ChatRoleV0::User,
    };

    // Create the chat request with a supported model
    let chat_request = ChatRequestV0 {
        model: "llama3.1:8b".to_string(), // Use supported model instead of gpt-4
        messages: vec![system_message, user_message],
    };

    ic_cdk::println!(
        "Calling LLM canister v0_chat with model: llama3.1:8b, query: {}",
        user_query
    );

    // Call the LLM canister using v0_chat method
    let llm_response: Result<(String,), _> =
        ic_cdk::call(llm_canister_id, "v0_chat", (chat_request,)).await;

    match llm_response {
        Ok((response_text,)) => {
            ic_cdk::println!("LLM response received: {}", response_text);

            // Try to parse the JSON response
            match serde_json::from_str::<QueryParseResult>(&response_text) {
                Ok(result) => {
                    ic_cdk::println!(
                        "Successfully parsed LLM response: table={}, query={}",
                        result.table,
                        result.query
                    );
                    Ok(result)
                }
                Err(parse_error) => {
                    ic_cdk::println!(
                        "Failed to parse LLM JSON response: {} | Error: {}",
                        response_text,
                        parse_error
                    );
                    // Fallback to manual parsing
                    parse_natural_language_query_fallback(user_query).await
                }
            }
        }
        Err(e) => {
            ic_cdk::println!("LLM canister call failed: {:?}", e);
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
                if let Ok(_) = id_value.parse::<i32>() {
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
    } else {
        "select=*".to_string()
    };

    Ok(QueryParseResult {
        table,
        query,
        error: None,
    })
}

// ... rest of the existing functions (fetch_from_supabase, insert_to_supabase, transform) remain the same ...

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
    let supabase_url = "https://tgsgxbmwhwcymfuodokl.supabase.co";
    let supabase_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InRnc2d4Ym13aHdjeW1mdW9kb2tsIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTA2NjY2ODQsImV4cCI6MjA2NjI0MjY4NH0.pkUJjiSEAzn3sG-C8iqdkyXPIWTHjypZ8HH31166uYM";

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

            if status_code >= 200 && status_code < 300 {
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
    let supabase_url = "https://tgsgxbmwhwcymfuodokl.supabase.co";
    let supabase_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InRnc2d4Ym13aHdjeW1mdW9kb2tsIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTA2NjY2ODQsImV4cCI6MjA2NjI0MjY4NH0.pkUJjiSEAzn3sG-C8iqdkyXPIWTHjypZ8HH31166uYM";

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
            if status_code >= 200 && status_code < 300 {
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
ic_cdk::export_candid!();
