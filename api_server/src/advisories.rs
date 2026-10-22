use serde::Serialize;

use json::JsonValue;
use std::error::Error;

use std::sync::Arc;
use tokio::sync::RwLock;

use tokio::time::{sleep, Duration};

use crate::config::Config;

// ============================================

#[derive(Serialize, Clone)]
pub struct Advisory {
    pub title: String,
    pub slug: String,
    pub lead: String,
    pub description: String,
    pub publication_date: String,
}
impl Advisory {
    fn from_json(advisory: &JsonValue) -> Option<Self> {
        let is_for_users = advisory["categories"].members().any(|category| { category["name"] == "Dla użytkowników" });

        if is_for_users {
            let title = advisory["title"].to_string();
            let slug = advisory["slug"].to_string();
            let lead = advisory["lead"].to_string();
            let description = advisory["description"].to_string();
            let publication_date = advisory["publication_date"].to_string();

            Some(
                Self { title, slug, lead, description, publication_date }
            )
        } 
        else {
            None
        }
    }
}

pub async fn updater(advisories: Arc<RwLock<Vec<Advisory>>>, config: Arc<Config>) {
    loop {
        {
            let mut new_advisories: Vec<Advisory> = vec![]; /* Vector of advisories that will replace old one if creating it ends successfully */

            match call_api(config.advisory_pages).await {
                Ok(pages) => {
                    let combined_list = join_pages(pages);
                    let members = combined_list.members();

                    for member in members {
                        if let Some(advisory) = Advisory::from_json(member) {
                            new_advisories.push(advisory);
                        }
                    }

                    let mut advisories_writer = advisories.write().await; /* Write guard for original advisory list */
                    *advisories_writer = new_advisories;
                    drop(advisories_writer);

                    log_ok!("Successfully called CERT.PL API and updated list of advisories!")

                },
                Err(e) => {
                    log_err!("Failed to call CERT.PL API with error: {e}.\nAdvisory list not updated.")
                }
            }
        }

        sleep(Duration::from_secs(config.data_update_interval)).await;
    }
}

fn join_pages(pages: Vec<JsonValue>) -> JsonValue {
    let mut joined: JsonValue = json::array![];

    for page in pages {
        let members = page["results"].members();
        for member in members {
            let _ = joined.push(member.clone());
        }
    }

    joined
}

async fn call_api(page_count: u16) -> Result<Vec<JsonValue>, Box<dyn Error + Send + Sync>> {
    let mut pages: Vec<JsonValue> = vec![];
    
    for i in 1..(page_count+1) {
        pages.push(
            json::parse(
                reqwest::get(
                    format!("https://moje.cert.pl/api/v1/advisories/?page={i}")
                )
                .await?
                .text()
                .await?
                .as_str()
            )?
        );
    }

    Ok(pages)
}