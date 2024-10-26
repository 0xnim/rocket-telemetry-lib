use crate::ProtocolError;
use std::convert::TryFrom;

// src/common/types.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    Telemetry = 0,
    Command = 1,
    Status = 2,
    Response = 3,
}

impl TryFrom<u8> for MessageType {
    type Error = ProtocolError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::Telemetry),
            1 => Ok(MessageType::Command),
            2 => Ok(MessageType::Status),
            3 => Ok(MessageType::Response),
            _ => Err(ProtocolError::InvalidMessageType),
        }
    }
}