use std::path::PathBuf;

pub struct Config {
    pub port: u16,
    pub advisory_pages: u16,
    pub advisories_update_interval: u64,
    pub log_responses: bool,
    pub msg_checker_keywords_path: Option<PathBuf>,
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

        let msg_checker_keywords_path: Option<PathBuf> = match std::env::var("MSG_CHECKER_KEYWORDS_PATH") {
            Ok(path) => Some(PathBuf::from(path)),
            Err(_) => {
                log_err!("No path to message checker keywords provided in enviromental variable. Defaulting to basic list of keywords embeded in binary.");
                None
            },
        };

        Self { port, advisory_pages, advisories_update_interval, log_responses, msg_checker_keywords_path }
    }
}