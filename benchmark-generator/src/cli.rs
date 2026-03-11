use clap::Parser;
use shared::serialization::format::MessageFormat;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(name = "benchmark-generator")]
#[command(about = "Low-Level Event Generator for RabbitMQ benchmark")]
pub struct BenchmarkCli {
    #[arg(long, default_value = "amqp://guest:guest@localhost:5672/%2f")]
    pub rabbitmq_url: String,

    #[arg(long, default_value = "benchmark.exchange")]
    pub exchange: String,

    #[arg(long, default_value = "json")]
    pub format: String,

    #[arg(long, default_value_t = 1000)]
    pub count: usize,

    #[arg(long, default_value_t = 0)]
    pub delay_ms: u64,
}

impl BenchmarkCli {
    pub fn parse_args() -> Self {
        <Self as Parser>::parse()
    }

    pub fn message_format(&self) -> Result<MessageFormat, String> {
        MessageFormat::from_str(&self.format)
    }
}