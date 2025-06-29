// Configuration module for API keys and external service URLs
// Uses environment variables at compile time

pub struct Config;

impl Config {
    pub fn supabase_url() -> Result<&'static str, &'static str> {
        option_env!("SUPABASE_URL").ok_or("SUPABASE_URL environment variable not set")
    }

    pub fn supabase_anon_key() -> Result<&'static str, &'static str> {
        option_env!("SUPABASE_ANON_KEY").ok_or("SUPABASE_ANON_KEY environment variable not set")
    }
}
