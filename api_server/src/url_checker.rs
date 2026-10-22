use std::{collections::HashSet, error::Error, sync::Arc, time::Duration};

use serde::Serialize;
use tokio::{sync::RwLock, time::sleep};
use url::Url;

use crate::config::Config;

pub async fn updater(domain_list: Arc<RwLock<HashSet<String>>>, config: Arc<Config>) {
    loop {
        {
            match call_api().await {
                Ok(domains) => {
                    // Convert domains vector to a hashset
                    let domains_hashset: HashSet<String> = domains.into_iter().collect();

                    // Get the lock
                    let mut domains_writer = domain_list.write().await;
                    // Write to the list
                    *domains_writer = domains_hashset;
                    // Drop the lock
                    drop(domains_writer);

                    log_ok!("Successfully called CERT.PL HOLE API and updated list of dangerous websites.");

                },
                Err(e) => {
                    log_err!("Failed to call CERT.PL HOlE API with error: {e}\nDomain list not updated.")
                }
            }
        }

        sleep(Duration::from_mins(config.data_update_interval)).await;
    }
}


async fn call_api() -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    
    let response = reqwest::get("https://hole.cert.pl/domains/v2/domains.json")
        .await?
        .text()
        .await?;

    let parsed_json: serde_json::Value = serde_json::from_str(&response)?;

    let domains: Vec<String> = parsed_json
        .as_array()
        .ok_or("Expected a JSON array")?
        .iter()
        .filter_map(|entry| {
            entry.get("DomainAddress")?
                .as_str()
                .map(String::from)
        })
        .collect();    

    Ok(domains)
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UrlState {
    Safe,
    Dangerous,
    Invalid,
}
pub async fn check_url(url: &str, domains: Arc<RwLock<HashSet<String>>>) -> UrlState {
    let domain_list = domains.read().await;

    let parsed_url = match Url::parse(url) {
        Ok(u) => u,
        Err(_) => return UrlState::Invalid,
    };

    let host = parsed_url.host_str();

    match host {
        Some(h) if domain_list.contains(h) => UrlState::Dangerous,
        Some(_) => UrlState::Safe,
        None => UrlState::Invalid,
    }

}