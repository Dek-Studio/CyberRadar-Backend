use std::sync::Arc;
use tokio::sync::RwLock;

use tokio::task;

use axum::{
    Router, routing::{get, post}
};
use crate::config::Config;

// ============================================

#[macro_use]
mod macros;

mod config;
mod advisories;
mod url_checker;
mod msg_checker;
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

    let msg_checker_state = Arc::new(
        msg_checker::MsgCheckerState::create( &config ).unwrap_or_else(|e| panic!("\x1b[41mFailed to create message checker state with error\n{e}\x1b[0m"))
    );

    println!("\n
    === STARTING API SERVER WITH THE FOLLOWING CONFIG: ===
    - PORT: {}
    - ADVISORY PAGES: {}
    - ADVISORIES UPDATE INTERVAL: {}
    - LOG RESPONSES: {}
    - MSG CHECKER KEYWORDS DICTIONARY PATH: {:?}
    \n",
        config.port, config.advisory_pages, config.data_update_interval, config.log_responses, config.msg_checker_keywords_path
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
        )
        .route("/check_msg", post({
                let msg_checker_state_clone = Arc::clone(&msg_checker_state);
                let config_clone = Arc::clone(&config);
                move |body: String| endpoints::check_msg(msg_checker_state_clone, config_clone, body)
            })
        );


    // Bind a listener on specified port for all addresses
    let listener = tokio::net::TcpListener::bind( 
        format!("0.0.0.0:{}", config.port))
        .await
        .unwrap_or_else(|e| 
            panic!("\x1b[41mFailed to bind port {} with error:\n{}\x1b[0m", config.port, e)
        );


    log_ok!("Server successfully bond and listening on port {}!", config.port);


    axum::serve(listener, app).await.unwrap(); /* I think it can be unwrapped since it shouldn't ever fail nor succeed */
}