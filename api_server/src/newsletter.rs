use json::{self, JsonValue};


pub async fn fetch_cert_api() -> (JsonValue, JsonValue, JsonValue) {
    let page_1 = reqwest::get("https://moje.cert.pl/api/v1/advisories/").await.unwrap();
    let page_2 = reqwest::get("https://moje.cert.pl/api/v1/advisories/?page=2").await.unwrap();
    let page_3 = reqwest::get("https://moje.cert.pl/api/v1/advisories/?page=3").await.unwrap();

    (
        json::parse(&page_1.text().await.unwrap_or_default().as_str() ).unwrap_or_else(|_| default_advisory_json() ),
        json::parse(&page_2.text().await.unwrap_or_default().as_str() ).unwrap_or_else(|_| default_advisory_json() ),
        json::parse(&page_3.text().await.unwrap_or_default().as_str() ).unwrap_or_else(|_| default_advisory_json() ),
    )
}

fn default_advisory_json() -> JsonValue {
    json::object! {
        "count": 0,
        "next": null,
        "previous": null,
        "results": json::array![]
    }
}

pub async fn newsletter_json() -> String {
    String::from("Newsletter goes here!")
}