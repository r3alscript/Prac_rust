use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    Validation(String),
    RabbitMq(String),
    Database(String),
    Serialization(String),
    SerdeJson(serde_json::Error),
    ProtobufEncode(prost::EncodeError),
    ProtobufDecode(prost::DecodeError),
    InvalidFormat(String),
    Message(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(msg) => write!(f, "Validation error: {}", msg),
            Self::RabbitMq(msg) => write!(f, "RabbitMQ error: {}", msg),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            Self::SerdeJson(err) => write!(f, "JSON serialization error: {}", err),
            Self::ProtobufEncode(err) => write!(f, "Protobuf encode error: {}", err),
            Self::ProtobufDecode(err) => write!(f, "Protobuf decode error: {}", err),
            Self::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            Self::Message(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for AppError {}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl From<prost::EncodeError> for AppError {
    fn from(value: prost::EncodeError) -> Self {
        Self::ProtobufEncode(value)
    }
}

impl From<prost::DecodeError> for AppError {
    fn from(value: prost::DecodeError) -> Self {
        Self::ProtobufDecode(value)
    }
}