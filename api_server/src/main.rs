use axum::{
    routing::get,
    Router
};
use crate::{config::Config};

mod config;
mod newsletter;

#[tokio::main]
async fn main() {
    let config = Config::fetch();

    println!(" === STARTING API SERVER WITH THE FOLLOWING CONFIG: ===\nPORT: {}", config.port);

    let app = Router::new()
        .route("/advisories.json", get(|| newsletter::newsletter_json()));

    /* Bind a listener on specified port for all addresses */
    let listener = tokio::net::TcpListener::bind( 
        format!("0.0.0.0:{}", config.port))
        .await
        .unwrap_or_else(|e| 
            panic!("Failed to bind port {} with error:\n{}", config.port, e)
        );

    println!("Server succesfully bond and listening on port {}!", config.port);

    axum::serve(listener, app).await.unwrap(); /* I think it can be unwrapped since it shouldn't ever fail nor succeed */
}