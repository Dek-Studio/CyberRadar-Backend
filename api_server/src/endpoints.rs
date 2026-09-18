use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    advisories::Advisory,
    config::Config
};

// ============================================

pub async fn advisories(advisories: Arc<RwLock<Vec<Advisory>>>, config: Arc<Config>) -> String {
    let guard = advisories.read().await;

    match serde_json::to_string(&*guard) {
        Ok(response) => {
            if config.log_responses {
                log_response!("advisories.json")
            }

            response
        },
        Err(e) => {
            log_err!("Failed to serialize advisories vector to string with error:\n{e}\nThis can be serious, consider restarting the server.");
            String::new()
        }
    }
}