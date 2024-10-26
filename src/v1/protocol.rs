// src/v1/protocol.rs

use crate::common::types::MessageType;
use crate::v1::packet::V1Packet;
use crate::packet::Packet;
use crate::v1::telemetry::TelemetryData;
use crate::v1::packet::PacketPayload;
use crate::ProtocolError;
use std::convert::TryFrom;
use crate::v1::telemetry::process_telemetry;
use bytes::{BytesMut, BufMut};

pub fn process_packet(data: &[u8]) -> Result<Packet, crate::ProtocolError> {
    println!("{:?}", data);

    let version = data[0] & 0b01111111;
    let type_ = data[1] & 0b11000000; // First 2 bits
    let message_type = crate::common::types::MessageType::try_from(type_);
    let id = data[1] & 0b00111111; // Last 6 bits
    let len = u16::from_be_bytes([data[2], data[3]]);
    let time = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    let data = data[8..].to_vec();

    let sliced_data = &data[0..];
    let mut bytes_mut = BytesMut::with_capacity(sliced_data.len());
    bytes_mut.extend_from_slice(sliced_data);

    let mut payload: Option<PacketPayload> = None; // Declare as Option

    if let Ok(MessageType::Telemetry) = message_type {
        let telemetry_data: TelemetryData = process_telemetry(&bytes_mut)?;
        payload = Some(PacketPayload::Telemetry(telemetry_data)); // Assign within the block
        println!("{:?}", payload);
    }

    let payload = payload.ok_or_else(|| {
        // Return an error if payload wasn't set
        // Adjust the error type and message as needed
        ProtocolError::PayloadNotInitialized
    })?;



    Ok(Packet::V1(V1Packet {
        version,
        message_type: message_type?,
        rocket_id: id,
        timestamp: time,
        payload,
    }))

}