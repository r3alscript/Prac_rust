use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub api_host: String,
    pub api_port: u16,

    pub rabbitmq_url: String,
    pub bid_exchange: String,
    pub bid_queue: String,
    pub bid_routing_key: String,

    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            api_host: env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            api_port: env::var("API_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("API_PORT must be a valid u16"),

            rabbitmq_url: env::var("RABBITMQ_URL")
                .expect("RABBITMQ_URL must be set"),

            bid_exchange: env::var("BID_EXCHANGE")
                .unwrap_or_else(|_| "bid.exchange".to_string()),

            bid_queue: env::var("BID_QUEUE")
                .unwrap_or_else(|_| "bid.placed.queue".to_string()),

            bid_routing_key: env::var("BID_ROUTING_KEY")
                .unwrap_or_else(|_| "bid.placed".to_string()),

            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
        }
    }

    pub fn api_addr(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }
}