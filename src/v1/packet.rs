use crate::v1::types::TelemetryData;
use crate::common::types::MessageType;
use bytes::{Buf, BytesMut, BufMut};

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

// impl packetpayload to binary

impl PacketPayload {
    pub fn to_binary(&self) -> Vec<u8> {
        match self {
            PacketPayload::Telemetry(data) => data.to_binary(),
            //PacketPayload::Command(data) => data.to_binary(),
            //PacketPayload::Status(data) => data.to_binary(),
            //PacketPayload::Response(data) => data.to_binary(),
        }
    }
}
