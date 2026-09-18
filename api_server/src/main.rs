use std::sync::Arc;
use tokio::sync::RwLock;

use tokio::task;

use axum::{
    routing::get,
    Router
};
use crate::config::Config;

// ============================================

#[macro_use]
mod macros;

mod config;
mod advisories;
mod endpoints;

// ============================================

#[tokio::main]
async fn main() {
    // App config from env
    let config = Arc::new(
        Config::fetch()
    );
    // ARC vector of advisories
    let advisories: Arc<RwLock<Vec<advisories::Advisory>>> = Arc::new(
        RwLock::new(
            vec![]
        )
    );

    println!("\n\n === STARTING API SERVER WITH THE FOLLOWING CONFIG: ===\n- PORT: {}\n- ADVISORY PAGES: {}\n- ADVISORIES UPDATE INTERVAL: {}\n- LOG RESPONSES: {}\n\n",
        config.port, config.advisory_pages, config.advisories_update_interval, config.log_responses
    );

    // Spawn background task for updating advisories list
    task::spawn(
        advisories::updater(
            Arc::clone(&advisories),
            Arc::clone(&config)
        )
    );


    let app = Router::new()
        .route("/advisories.json", get({
                let config_clone = Arc::clone(&config);
                move || endpoints::advisories(Arc::clone(&advisories), config_clone )
            })
        );


    // Bind a listener on specified port for all addresses
    let listener = tokio::net::TcpListener::bind( 
        format!("0.0.0.0:{}", config.port))
        .await
        .unwrap_or_else(|e| 
            panic!("Failed to bind port {} with error:\n{}", config.port, e)
        );


    log_ok!("Server successfully bond and listening on port {}!", config.port);


    axum::serve(listener, app).await.unwrap(); /* I think it can be unwrapped since it shouldn't ever fail nor succeed */
}