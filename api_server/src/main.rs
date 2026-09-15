use axum::{
    routing::get,
    Router
};
use crate::config::Config;

mod config;
mod newsletter;

#[tokio::main]
async fn main() {
    let config = Config::fetch();

    println!("STARTING API SERVER WITH THE FOLLOWING CONFIG:\nPORT: {}", config.port);

    let app = Router::new()
        .route("/newsletter.json", get(|| newsletter::newsletter_json()));

    let listener = tokio::net::TcpListener::bind( /* Bind a listener on specified port for all addresses */
        format!("0.0.0.0:{}", config.port))
        .await
        .unwrap_or_else(|e| 
            panic!("Failed to bind port {} with error:\n{}", config.port, e)
        );

    axum::serve(listener, app).await.unwrap(); /* I think it can be unwrapped since it shouldn't ever fail nor succeed */
}