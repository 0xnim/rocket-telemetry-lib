use crate::v1::telemetry::TelemetryData;
use crate::common::types::MessageType;

#[derive(Debug)]
pub struct V1Packet {
    pub version: u8,
    pub message_type: MessageType,
    pub rocket_id: u8,
    pub timestamp: u32,
    pub payload: PacketPayload,
}

#[derive(Debug)]
pub enum PacketPayload {
    Telemetry(TelemetryData),
    //Command(CommandData),
    //Status(StatusData),
    //Response(ResponseData),
}