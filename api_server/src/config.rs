pub struct Config {
    pub port: String,
}
impl Config {
    pub fn fetch() -> Self {
        let port = std::env::var("PORT").unwrap_or_else(|_| {
            eprintln!("No port provided in environmental variable. Defaulting to 3000");
            String::from("3000")
        });

        Self { port }
    }
}