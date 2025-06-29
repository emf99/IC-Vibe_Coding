// Configuration module for API keys and external service URLs
// Uses environment variables at compile time

pub struct Config;

impl Config {
    pub fn supabase_url() -> &'static str {
        option_env!("SUPABASE_URL")
            .unwrap_or("https://tgsgxbmwhwcymfuodokl.supabase.co")
    }
    
    pub fn supabase_anon_key() -> &'static str {
        option_env!("SUPABASE_ANON_KEY")
            .unwrap_or("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InRnc2d4Ym13aHdjeW1mdW9kb2tsIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTA2NjY2ODQsImV4cCI6MjA2NjI0MjY4NH0.pkUJjiSEAzn3sG-C8iqdkyXPIWTHjypZ8HH31166uYM")
    }
}
