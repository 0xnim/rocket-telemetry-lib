// src/v1/telemetry.rs
use crate::error::ProtocolError;
use bytes::{Buf, BytesMut};

#[derive(Debug)]
pub struct TelemetryData {
    pub altitude: Option<f32>,
    pub pressure: Option<f32>,
    pub rotation: Option<(f32, f32, f32)>,
    pub acceleration: Option<(f32, f32, f32)>,
    pub temperature: Option<f32>,
    pub gps: Option<GpsData>,
    pub time: Option<u32>,
    // ... other fields
}

// can happen that only some fields are present
#[derive(Debug)]
pub struct GpsData {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
    pub ground_speed: f32,
    pub heading: f32,
}

pub fn process_telemetry(buf: &BytesMut) -> Result<TelemetryData, ProtocolError> {
    let mut data = TelemetryData {
        altitude: None,
        pressure: None,
        rotation: None,
        acceleration: None,
        temperature: None,
        gps: Some(GpsData {
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            ground_speed: 0.0,
            heading: 0.0,
        }),
        time: None,
    };

    let mut remaining = buf.clone();
    while remaining.has_remaining() {
        let field_id = remaining.get_u8();
        match field_id {
            0x01 => data.altitude = Some(remaining.get_u16() as f32),
            0x02 => data.pressure = Some(remaining.get_u16() as f32),
            0x03..=0x05 => {
                let x = remaining.get_u16() as f32;
                let y = remaining.get_u16() as f32;
                let z = remaining.get_u16() as f32;
                data.rotation = Some((x, y, z));
            },
            0x06..=0x08 => {
                let x = remaining.get_u16() as f32;
                let y = remaining.get_u16() as f32;
                let z = remaining.get_u16() as f32;
                data.acceleration = Some((x, y, z));
            },
            0x09 => data.temperature = Some(remaining.get_u16() as f32),
            // Make sure that only the fields that are present are processed, all fields should be processed seperately
            0x0A => data.gps.as_mut().unwrap().latitude = remaining.get_u32() as f32,
            0x0B => data.gps.as_mut().unwrap().longitude = remaining.get_u32() as f32,
            0x0C => data.gps.as_mut().unwrap().altitude = remaining.get_u32() as f32,
            0x0D => data.gps.as_mut().unwrap().ground_speed = remaining.get_u32() as f32,
            0x0E => data.gps.as_mut().unwrap().heading = remaining.get_u32() as f32,
            0x0F => data.time = Some(remaining.get_u32()),

            // ... handle other fields
            _ => return Err(ProtocolError::InvalidFieldId(field_id)),
        }
    }

    Ok(data)
}