use std::{fs, sync::Arc};
use serde::Deserialize;

use crate::config::Config;

// ============================================

const DEFAULT_KEYWORDS: &str = include_str!("../../keywords.ron");

// ============================================


pub struct MsgCheckerState {
    keywords: Vec<Keyword>,
}
impl MsgCheckerState {
    pub fn create(config: &Config) -> Result<Self, String> {

        let keywords = match &config.msg_checker_keywords_path {
            Some(path) => ron::from_str(&fs::read_to_string(path).map_err(|e| format!("Failed to read keywords file to string with error: {e}"))?).map_err(|e| format!("Failed to deserialize keywords file with error: {e}"))?,
            None => ron::from_str(DEFAULT_KEYWORDS).map_err(|e| format!("Failed to deserialize default keywords embeded in binary with error: {e}"))?,
        };

        Ok( Self { keywords } )
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Keyword {
    value: String,
    weight: u32,
    category: String,
}


pub fn check_msg(msg: &str, state: Arc<MsgCheckerState>) -> (u32, Vec<String>) {
    
    let mut found_categories: Vec<&str> = vec![];
    let mut total: u32 = 0;

    let msg = msg.to_lowercase();

    // Check if each keyword is contained in message, if it is - add its weight to total and push its category to categories
    for keyword in &state.keywords {
        if msg.contains( &keyword.value.as_str() ) {
            total += &keyword.weight;
            if !found_categories.contains(&keyword.category.as_str()) { found_categories.push(&keyword.category) }
        }
    }

    // Add 3 points if message containts a URL
    let contains_urls = contains_urls(&msg);
    if contains_urls { total += 3 }

    if found_categories.contains(&"presja_czasu_naganianie") && found_categories.contains(&"bankowosc_finanse") {
        total += 3;
    }
    
    if found_categories.contains(&"presja_czasu_naganianie") && found_categories.contains(&"doplata_paczka") {
        total += 2;
    }
    if found_categories.contains(&"podstepne_wiado_znajomi") && contains_urls {
        total += 3;
    }

    // Return
    (
        total,
        found_categories.iter().map(|&c| c.to_string()).collect()
    )    
}

fn contains_urls(msg: &str) -> bool {
    // TODO: Make this more refined than just checking for protocol prefix!!

    if msg.contains("https://") { return true }
    if msg.contains("http://") { return true }

    false
}