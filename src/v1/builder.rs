// src/v1/builder.rs

/// Builder for V1 packets
/// Implements the builder pattern
/// Allows for easy creation of packets, with returning the binary data

use crate::common::types::MessageType;
use crate::v1::types::{TelemetryData, TelemetryField};
use crate::v1::packet::PacketPayload;
use crate::v1::packet::V1Packet;
use bytes::{BytesMut, BufMut};

// Packet builder struct = V1Packet builder struct

impl V1Packet {
    pub fn new(message_type: MessageType, rocket_id: u8, timestamp: u32, payload: PacketPayload) -> Self {
        V1Packet {
            version: 1,
            message_type,
            rocket_id,
            timestamp,
            payload,
        }
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let len = 8+self.payload.to_binary().len();
        let mut bytes_mut = BytesMut::with_capacity(len);
        bytes_mut.put_u8(self.version);
        let message_type = self.message_type.clone() as u8;
        let rocket_id = self.rocket_id as u8;
        bytes_mut.put_u8((message_type << 6) | rocket_id);
        bytes_mut.put_u16(len as u16);
        bytes_mut.put_u32(self.timestamp);
        // split into bytes buf does not work for vec8
        bytes_mut.put_slice(&self.payload.to_binary());
        bytes_mut.to_vec()
    }
}

impl PacketPayload {
    pub fn new(telemetry_data: TelemetryData) -> Self {
        PacketPayload::Telemetry(telemetry_data)
    }
}