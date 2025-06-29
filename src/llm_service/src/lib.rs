use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub role: ChatRole,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChatRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct QueryParseResult {
    pub table: String,
    pub query: String,
    pub error: Option<String>,
}

// Główna funkcja do parsowania natural language na SQL
#[ic_cdk::update]
async fn parse_natural_language_to_sql(user_query: String) -> Result<QueryParseResult, String> {
    ic_cdk::println!("Parsing query: {}", user_query);

    // Stwórz prompt systemowy dla SQL parsing
    let system_prompt = r#"You are a SQL query generator for a PostgreSQL database accessed via Supabase REST API.

Database schema:
- todos: id (integer), title (text), description (text), is_done (boolean), due_date (timestamp), status (text), created_at (timestamp)
- users: id (integer), name (text), email (text), created_at (timestamp)  
- posts: id (integer), title (text), content (text), user_id (integer), created_at (timestamp)

Convert natural language to Supabase PostgREST format:
- "select=*" for all columns
- "select=id,title" for specific columns  
- "is_done=eq.true" for boolean filters
- "due_date=not.is.null" for non-null filters
- "due_date=is.null" for null filters
- "title=ilike.*search*" for text search (ALWAYS use asterisks * not percent signs %)

IMPORTANT: For text search, ALWAYS use asterisks (*) format: "title=ilike.*word*"
NEVER use percent signs (%) format: "title=ilike.%word%"

Respond ONLY with JSON in this exact format:
{"table": "table_name", "query": "supabase_query_string", "error": null}

Examples:
"get all todos" → {"table": "todos", "query": "select=*", "error": null}
"show completed todos" → {"table": "todos", "query": "select=*&is_done=eq.true", "error": null}
"find incomplete todos" → {"table": "todos", "query": "select=*&is_done=eq.false", "error": null}
"show todos with title like dog" → {"table": "todos", "query": "select=*&title=ilike.*dog*", "error": null}
"find todos containing work" → {"table": "todos", "query": "select=*&title=ilike.*work*", "error": null}"#;

    // Przygotuj wiadomości dla Groq
    let messages = vec![
        ChatMessage {
            content: system_prompt.to_string(),
            role: ChatRole::System,
        },
        ChatMessage {
            content: format!("Parse this query: {}", user_query),
            role: ChatRole::User,
        },
    ];

    // Spróbuj wywołać Groq API
    match call_groq_api(messages).await {
        Ok(llm_response) => {
            // Sparsuj odpowiedź JSON
            match serde_json::from_str::<QueryParseResult>(&llm_response) {
                Ok(result) => {
                    ic_cdk::println!(
                        "Successfully parsed via Groq: table={}, query={}",
                        result.table,
                        result.query
                    );
                    Ok(result)
                }
                Err(_) => {
                    ic_cdk::println!("Failed to parse Groq response, using fallback");
                    parse_query_smart_fallback(user_query).await
                }
            }
        }
        Err(error) => {
            ic_cdk::println!("Groq API failed: {}, using fallback", error);
            parse_query_smart_fallback(user_query).await
        }
    }
}

// Wywołanie Groq API dla bardzo szybkiego LLM
async fn call_groq_api(messages: Vec<ChatMessage>) -> Result<String, String> {
    let api_url = "https://api.groq.com/openai/v1/chat/completions";

    // Use environment variable for Groq API key
    let groq_api_key = option_env!("GROQ_API_KEY")
        .ok_or("GROQ_API_KEY environment variable not set")?;

    // Przygotuj payload dla Groq API
    let payload = serde_json::json!({
        "model": "llama-3.1-8b-instant", // Bardzo szybki model Groq
        "messages": messages.iter().map(|msg| {
            serde_json::json!({
                "role": match msg.role {
                    ChatRole::System => "system",
                    ChatRole::User => "user",
                    ChatRole::Assistant => "assistant"
                },
                "content": msg.content
            })
        }).collect::<Vec<_>>(),
        "temperature": 0.1,
        "max_tokens": 300,
        "top_p": 1.0,
        "stream": false
    });

    ic_cdk::println!("Calling Groq API with model: llama-3.1-8b-instant");

    // Wykonaj HTTP request do Groq
    let request = CanisterHttpRequestArgument {
        url: api_url.to_string(),
        method: HttpMethod::POST,
        body: Some(payload.to_string().into_bytes()),
        max_response_bytes: Some(2048),
        transform: Some(TransformContext::from_name(
            "transform".to_string(),
            serde_json::json!({}).to_string().into_bytes(),
        )),
        headers: vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {}", groq_api_key),
            },
        ],
    };

    match http_request(request, 25_000_000_000).await {
        Ok((response,)) => {
            let response_body = String::from_utf8(response.body)
                .map_err(|_| "Invalid response encoding".to_string())?;

            ic_cdk::println!("Groq API response: {}", response_body);

            // Sparsuj odpowiedź Groq API
            let api_response: serde_json::Value = serde_json::from_str(&response_body)
                .map_err(|_| "Failed to parse Groq API response".to_string())?;

            let content = api_response["choices"][0]["message"]["content"]
                .as_str()
                .ok_or("No content in Groq API response")?;

            Ok(content.to_string())
        }
        Err((code, message)) => {
            ic_cdk::println!("Groq HTTP request failed: {} - {}", code as u32, message);
            Err(format!("Groq API call failed: {}", message))
        }
    }
}

// Bardzo inteligentny fallback parser bez potrzeby zewnętrznego LLM
async fn parse_query_smart_fallback(user_query: String) -> Result<QueryParseResult, String> {
    let query_lower = user_query.to_lowercase();
    ic_cdk::println!("Smart parsing: {}", query_lower);

    // Rozpoznaj tabelę
    let table = if query_lower.contains("todo") || query_lower.contains("task") {
        "todos"
    } else if query_lower.contains("user") {
        "users"
    } else if query_lower.contains("post") {
        "posts"
    } else {
        // Domyślnie todos dla zapytań ogólnych
        "todos"
    };

    // Zbuduj zapytanie Supabase
    let mut query_parts: Vec<String> = vec![];

    // Określ kolumny do wyboru
    if query_lower.contains("only id") || query_lower.contains("just id") {
        query_parts.push("select=id".to_string());
    } else if query_lower.contains("only title") || query_lower.contains("just title") {
        query_parts.push("select=title".to_string());
    } else if query_lower.contains("id and title") {
        query_parts.push("select=id,title".to_string());
    } else {
        query_parts.push("select=*".to_string());
    }

    // Filtry dla todos
    if table == "todos" {
        // Status filters
        if query_lower.contains("completed")
            || query_lower.contains("done")
            || query_lower.contains("finished")
        {
            query_parts.push("is_done=eq.true".to_string());
        } else if query_lower.contains("incomplete")
            || query_lower.contains("not done")
            || query_lower.contains("pending")
            || query_lower.contains("unfinished")
        {
            query_parts.push("is_done=eq.false".to_string());
        }

        // Due date filters
        if query_lower.contains("with due date")
            || (query_lower.contains("due") && !query_lower.contains("no due"))
        {
            query_parts.push("due_date=not.is.null".to_string());
        } else if query_lower.contains("no due date") || query_lower.contains("without due date") {
            query_parts.push("due_date=is.null".to_string());
        }

        // Specific status
        if query_lower.contains("status") {
            if query_lower.contains("active") {
                query_parts.push("status=eq.active".to_string());
            } else if query_lower.contains("archived") {
                query_parts.push("status=eq.archived".to_string());
            }
        }

        // Text search in title - handle specific patterns first
        if query_lower.contains("title contains") || query_lower.contains("title like") {
            // Extract the search term after "title contains" or "title like"
            let patterns = ["title contains ", "title like "];
            for pattern in &patterns {
                if let Some(pos) = query_lower.find(pattern) {
                    let remainder = &query_lower[pos + pattern.len()..];
                    let search_term = if let Some(word_end) = remainder.find(' ') {
                        remainder[..word_end].to_string()
                    } else {
                        remainder.to_string()
                    };
                    if !search_term.is_empty() {
                        query_parts.push(format!("title=ilike.*{}*", search_term));
                        break;
                    }
                }
            }
        }
        // General text search fallback
        else if let Some(search_term) = extract_search_term(&query_lower) {
            query_parts.push(format!("title=ilike.*{}*", search_term));
        }

        // ID specific queries
        if let Some(id) = extract_id(&query_lower) {
            query_parts.push(format!("id=eq.{}", id));
        }
    }

    // Sortowanie
    if query_lower.contains("latest")
        || query_lower.contains("newest")
        || query_lower.contains("recent")
    {
        query_parts.push("order=created_at.desc".to_string());
    } else if query_lower.contains("oldest") || query_lower.contains("first") {
        query_parts.push("order=created_at.asc".to_string());
    }

    // Limit
    if query_lower.contains("first 5") || query_lower.contains("top 5") {
        query_parts.push("limit=5".to_string());
    } else if query_lower.contains("first 10") || query_lower.contains("top 10") {
        query_parts.push("limit=10".to_string());
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
    // Proste parsowanie terminu w cudzysłowach
    if let Some(start) = query.find('"') {
        if let Some(end) = query[start + 1..].find('"') {
            return Some(query[start + 1..start + 1 + end].to_string());
        }
    }

    // Szukaj po słowach kluczowych
    for keyword in &[
        "with ",
        "containing ",
        "contains ",
        "about ",
        "titled ",
        "named ",
        "like ",
    ] {
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

// Funkcja pomocnicza do wyciągania ID
fn extract_id(query: &str) -> Option<u32> {
    // Szukaj "id 1", "with id 5", etc.
    for keyword in &["id ", "with id ", "having id "] {
        if let Some(pos) = query.find(keyword) {
            let remainder = &query[pos + keyword.len()..];
            if let Some(word_end) = remainder.find(' ') {
                if let Ok(id) = remainder[..word_end].parse::<u32>() {
                    return Some(id);
                }
            } else if let Ok(id) = remainder.parse::<u32>() {
                return Some(id);
            }
        }
    }

    // Szukaj liczby na końcu zdania
    let words: Vec<&str> = query.split_whitespace().collect();
    if let Some(last_word) = words.last() {
        if let Ok(id) = last_word.parse::<u32>() {
            return Some(id);
        }
    }

    None
}

// Funkcja transformacji dla HTTP response
#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let mut headers = Vec::new();
    if let Some(h) = raw.response.headers.first() {
        headers.push(h.clone());
    }

    HttpResponse {
        status: raw.response.status,
        body: raw.response.body,
        headers,
    }
}

// Eksportuj interfejs Candid
ic_cdk::export_candid!();
