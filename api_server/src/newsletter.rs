use std::{collections::HashMap, time::{Duration, SystemTime, UNIX_EPOCH}};

pub struct NewsletterNews {
    pub title: String,
    pub date: String,
    pub content: String,
}
impl NewsletterNews {
    fn parse_news_email(raw_email: &str) -> Self {

        todo!("Parse email")
    }
}

pub struct NewsletterData {
    pub email_news: HashMap<u64, NewsletterNews>,
}
impl NewsletterData {
    pub fn update_email_news(&self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        println!("{now}");
        println!("Title: {}", a_news.title);
        println!("Date: {}", a_news.date);
        println!("Content: {}", a_news.content);
    }
}

pub async fn newsletter_json() -> String {
    String::from("Newsletter goes here!")
}