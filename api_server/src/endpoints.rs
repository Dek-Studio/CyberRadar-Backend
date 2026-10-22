use std::{collections::HashSet, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    advisories::Advisory, config::Config, msg_checker::{self, MsgCheckerState}, url_checker
};

// ============================================

pub async fn advisories(advisories: Arc<RwLock<Vec<Advisory>>>, config: Arc<Config>) -> String {
    let guard = advisories.read().await;

    match serde_json::to_string(&*guard) {
        Ok(response) => {
            if config.log_responses { log_response!("advisories.json") }
            response
        },
        Err(e) => {
            log_err!("Failed to serialize advisories vector to string with error:\n{e}\nThis can be serious, consider restarting the server.");
            String::new()
        }
    }
}

pub async fn check_msg(msg_checker_state: Arc<MsgCheckerState>, config: Arc<Config>, body: String) -> String {

    let (score, categories) = msg_checker::check_msg(&body, msg_checker_state);

    if config.log_responses { log_response!("check_msg") }

    serde_json::json!({
        "score": score,
        "categories": categories,
    }).to_string()
}

pub async fn check_url(dangerous_domains: Arc<RwLock<HashSet<String>>>, config: Arc<Config>, body: String) -> String {
    
    if config.log_responses { log_response!("check_url") }

    serde_json::json!({
        "status": &url_checker::check_url(&body, dangerous_domains).await
    }).to_string()
}