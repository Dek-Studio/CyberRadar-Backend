pub struct Config {
    pub port: u16,
    pub advisory_pages: u16,
    pub advisories_update_interval: u64,
    pub log_responses: bool,
}
impl Config {
    pub fn fetch() -> Self {
        let port: u16 = std::env::var("PORT").unwrap_or_default().parse().unwrap_or_else(|_| {
            log_err!("No port provided in environmental variable. Defaulting to 3000.");
            3000
        });

        let advisory_pages: u16 = std::env::var("ADVISORY_PAGES").unwrap_or_default().parse().unwrap_or_else(|_| {
            log_err!("No advisory pages count provided in enviromental variable. Defaulting to 3.");
            3
        });

        let advisories_update_interval: u64 = std::env::var("ADVISORIES_UPDATE_INTERVAL").unwrap_or_default().parse().unwrap_or_else(|_| {
            log_err!("No advisory update interval provided in enviromental variable. Defaulting to 30.");
            30
        });

        let log_responses = match std::env::var("LOG_RESPONSES").unwrap_or_else(|_| {
            log_err!("No log responses value provided in enviromental variable. Defaulting to true.");
            String::from("true")
        }).to_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => true,
        };

        Self { port, advisory_pages, advisories_update_interval, log_responses }
    }
}