use std::{array, error::Error};

use axum::Json;
use json::{self, JsonValue};
use serde::{Deserialize, Serialize};


pub async fn fetch_cert_api() -> Result<(JsonValue, JsonValue, JsonValue), Box<dyn Error>> {
    let page_1 = reqwest::get("https://moje.cert.pl/api/v1/advisories/").await?;
    let page_2 = reqwest::get("https://moje.cert.pl/api/v1/advisories/?page=2").await?;
    let page_3 = reqwest::get("https://moje.cert.pl/api/v1/advisories/?page=3").await?;

    Ok((
        json::parse(&page_1.text().await?.as_str() )?,
        json::parse(&page_2.text().await?.as_str() )?,
        json::parse(&page_3.text().await?.as_str() )?,
    ))
}

#[derive(Serialize, Debug)]
struct NewsletterNews {
    title: String,
    slug: String,
    lead: String,
    description: String,
}

fn combine_newsletter_pages(page1: JsonValue, page2: JsonValue, page3: JsonValue, ) -> JsonValue {
    let mut combined = json::array![];

    for page in &[page1, page2, page3] {
        let members = page["results"].members();
        for member in members {
            let _ = combined.push(member.clone());
        }
    }

    combined
}

pub async fn newsletter_json() -> String {
    match fetch_cert_api().await {
        Ok((page1, page2, page3)) => {
            let combined_json = combine_newsletter_pages(page1, page2, page3);
            let members = combined_json.members();

            let mut ready_news_vector: Vec<NewsletterNews> = vec![];

            for member in members {
                let is_for_uses = member["categories"].members().any(|category| { category["name"] == "Dla użytkowników" });
                
                if is_for_uses {
                    let title = member["title"].to_string();
                    let slug = member["slug"].to_string();
                    let lead = member["lead"].to_string();
                    let description = member["description"].to_string();

                    ready_news_vector.push(NewsletterNews { title, slug, lead, description });
                }
            } /* for member in members */

            serde_json::to_string(&ready_news_vector).unwrap_or_default()


        },
        Err(e) => {
            eprintln!("Failed to fetch CERT.PL API with error: {e}");
            return String::from("");
        }
    }
}