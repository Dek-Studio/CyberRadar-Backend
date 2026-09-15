pub struct Config {
    pub port: String,
}
impl Config {
    pub fn fetch() -> Self {
        let port = std::env::var("PORT").unwrap_or(String::from("3000"));

        Self { port }
    }
}