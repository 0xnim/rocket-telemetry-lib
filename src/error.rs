// src/error.rs
#[derive(Debug, Clone, thiserror::Error)]
pub enum ProtocolError {
    #[error("Invalid packet length")]
    InvalidLength,
    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(u8),
    #[error("Invalid message type")]
    InvalidMessageType,
    #[error("Invalid field ID: {0}")]
    InvalidFieldId(u8),
    #[error("Malformed packet")]
    MalformedPacket,
    #[error("Payload not initialized")]
    PayloadNotInitialized,
}