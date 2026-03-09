use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageFormat {
    Json,
    Protobuf,
}

impl MessageFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Protobuf => "protobuf",
        }
    }

    pub fn content_type(&self) -> &'static str {
        match self {
            Self::Json => "application/json",
            Self::Protobuf => "application/protobuf",
        }
    }

    pub fn routing_key(&self) -> &'static str {
        match self {
            Self::Json => "benchmark.json",
            Self::Protobuf => "benchmark.protobuf",
        }
    }
}

impl fmt::Display for MessageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for MessageFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "protobuf" | "proto" => Ok(Self::Protobuf),
            other => Err(format!("Unsupported message format: {}", other)),
        }
    }
}