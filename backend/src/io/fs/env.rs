pub mod keys {
    pub const LOOKSYK_CONFIG_PATH: &str = "LOOKSYK_CONFIG_PATH";
}

pub fn get_or_default(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => default.to_string(),
    }
}
