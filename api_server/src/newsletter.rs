use std::error::Error;

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

fn default_advisory_json() -> JsonValue {
    json::object! {
        "count": 0,
        "next": null,
        "previous": null,
        "results": json::array![]
    }
}

#[derive(Serialize)]
struct NewsletterResponse {
    title: String,
    slug: String,
    lead: String,
    description: String,
}

pub async fn newsletter_json() -> String {
    // let (page1, page2, page3) = fetch_cert_api().await;
    // format!("Page 1:\n{}\n\n\nPage2:\n{}\n\n\nPage3:\n{}", page1, page2, page3)

    let vector = vec![
        NewsletterResponse {
            title: String::from("titile1"),
            slug: String::from("slug1"),
            lead: String::from("lead1"),
            description: String::from("description1"),
        },
        NewsletterResponse {
            title: String::from("titile3"),
            slug: String::from("slug3"),
            lead: String::from("lead3"),
            description: String::from("description3"),
        },
        NewsletterResponse {
            title: String::from("titile2"),
            slug: String::from("slug2"),
            lead: String::from("lead2"),
            description: String::from("description2"),
        },
    ];

    serde_json::to_string(&vector).unwrap_or_default()
}