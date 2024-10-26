// src/v1/telemetry.rs
use crate::error::ProtocolError;
use bytes::{Buf, BytesMut, BufMut};
use crate::v1::types::{TelemetryField, TelemetryData, GpsData};

impl TelemetryData {
    pub fn new() -> Self {
        TelemetryData {
            altitude: None,
            pressure: None,
            rotation: None,
            acceleration: None,
            temperature: None,
            gps: Some(GpsData {
                latitude: None,
                longitude: None,
                altitude: None,
                ground_speed: None,
                heading: None,
            }),
            time: None,
        }
    }

    pub fn set_field(&mut self, field: TelemetryField, value: f32) {
        match field {
            TelemetryField::Altitude => self.altitude = Some(value),
            TelemetryField::Pressure => self.pressure = Some(value),
            TelemetryField::RotationX => {
                let (_, y, z) = self.rotation.unwrap_or((0.0, 0.0, 0.0));
                self.rotation = Some((value, y, z));
            }
            TelemetryField::RotationY => {
                let (x, _, z) = self.rotation.unwrap_or((0.0, 0.0, 0.0));
                self.rotation = Some((x, value, z));
            }
            TelemetryField::RotationZ => {
                let (x, y, _) = self.rotation.unwrap_or((0.0, 0.0, 0.0));
                self.rotation = Some((x, y, value));
            }
            TelemetryField::AccelerationX => {
                let (_, y, z) = self.acceleration.unwrap_or((0.0, 0.0, 0.0));
                self.acceleration = Some((value, y, z));
            }
            TelemetryField::AccelerationY => {
                let (x, _, z) = self.acceleration.unwrap_or((0.0, 0.0, 0.0));
                self.acceleration = Some((x, value, z));
            }
            TelemetryField::AccelerationZ => {
                let (x, y, _) = self.acceleration.unwrap_or((0.0, 0.0, 0.0));
                self.acceleration = Some((x, y, value));
            }
            TelemetryField::Temperature => self.temperature = Some(value),
            TelemetryField::GpsLatitude => self.gps.as_mut().unwrap().latitude = Some(value),
            TelemetryField::GpsLongitude => self.gps.as_mut().unwrap().longitude = Some(value),
            TelemetryField::GpsAltitude => self.gps.as_mut().unwrap().altitude = Some(value),
            TelemetryField::GpsGroundSpeed => self.gps.as_mut().unwrap().ground_speed = Some(value),
            TelemetryField::GpsHeading => self.gps.as_mut().unwrap().heading = Some(value),
            TelemetryField::Time => self.time = Some(value as u32),
        }
    }

    pub fn get_field(&self, field: TelemetryField) -> Option<f32> {
        match field {
            TelemetryField::Altitude => self.altitude,
            TelemetryField::Pressure => self.pressure,
            TelemetryField::RotationX => self.rotation.map(|(x, _, _)| x),
            TelemetryField::RotationY => self.rotation.map(|(_, y, _)| y),
            TelemetryField::RotationZ => self.rotation.map(|(_, _, z)| z),
            TelemetryField::AccelerationX => self.acceleration.map(|(x, _, _)| x),
            TelemetryField::AccelerationY => self.acceleration.map(|(_, y, _)| y),
            TelemetryField::AccelerationZ => self.acceleration.map(|(_, _, z)| z),
            TelemetryField::Temperature => self.temperature,
            TelemetryField::GpsLatitude => self.gps.as_ref().map(|x| x.latitude).flatten(),
            TelemetryField::GpsLongitude => self.gps.as_ref().map(|x| x.longitude).flatten(),
            TelemetryField::GpsAltitude => self.gps.as_ref().map(|x| x.altitude).flatten(),
            TelemetryField::GpsGroundSpeed => self.gps.as_ref().map(|x| x.ground_speed).flatten(),
            TelemetryField::GpsHeading => self.gps.as_ref().map(|x| x.heading).flatten(),
            TelemetryField::Time => self.time.map(|x| x as f32),
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        if self.altitude.is_some() {
            len += 2;
        }
        if self.pressure.is_some() {
            len += 2;
        }
        if self.rotation.is_some() {
            len += 6;
        }
        if self.acceleration.is_some() {
            len += 6;
        }
        if self.temperature.is_some() {
            len += 2;
        }
        if self.gps.is_some() {
            len += 12;
        }
        if self.time.is_some() {
            len += 4;
        }
        len
    }

    pub fn fields(&self) -> Vec<TelemetryField> {
        let mut fields = Vec::new();
        if self.altitude.is_some() {
            fields.push(TelemetryField::Altitude);
        }
        if self.pressure.is_some() {
            fields.push(TelemetryField::Pressure);
        }
        if self.rotation.is_some() {
            fields.push(TelemetryField::RotationX);
            fields.push(TelemetryField::RotationY);
            fields.push(TelemetryField::RotationZ);
        }
        if self.acceleration.is_some() {
            fields.push(TelemetryField::AccelerationX);
            fields.push(TelemetryField::AccelerationY);
            fields.push(TelemetryField::AccelerationZ);
        }
        if self.temperature.is_some() {
            fields.push(TelemetryField::Temperature);
        }
        if self.gps.is_some() {
            let gps = self.gps.as_ref().unwrap();
            if gps.latitude.is_some() {
                fields.push(TelemetryField::GpsLatitude);
            }
            if gps.longitude.is_some() {
                fields.push(TelemetryField::GpsLongitude);
            }
            if gps.altitude.is_some() {
                fields.push(TelemetryField::GpsAltitude);
            }
            if gps.ground_speed.is_some() {
                fields.push(TelemetryField::GpsGroundSpeed);
            }
            if gps.heading.is_some() {
                fields.push(TelemetryField::GpsHeading);
            }
        }
        if self.time.is_some() {
            fields.push(TelemetryField::Time);
        }
        fields
    }

    pub fn to_binary(&self) -> Vec<u8> {
        let mut bytes_mut = BytesMut::with_capacity(self.len());
        for field in self.fields() {
            let value = match self.get_field(field) {
                Some(v) => v,
                None => continue, // Skip this iteration if value is None
            };
            bytes_mut.put_u16(value as u16);
        }
        bytes_mut.to_vec()
    }
}

impl TryFrom<u8> for TelemetryField {
    type Error = ProtocolError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(TelemetryField::Altitude),
            0x02 => Ok(TelemetryField::Pressure),
            0x03 => Ok(TelemetryField::RotationX),
            0x04 => Ok(TelemetryField::RotationY),
            0x05 => Ok(TelemetryField::RotationZ),
            0x06 => Ok(TelemetryField::AccelerationX),
            0x07 => Ok(TelemetryField::AccelerationY),
            0x08 => Ok(TelemetryField::AccelerationZ),
            0x09 => Ok(TelemetryField::Temperature),
            0x0A => Ok(TelemetryField::GpsLatitude),
            0x0B => Ok(TelemetryField::GpsLongitude),
            0x0C => Ok(TelemetryField::GpsAltitude),
            0x0D => Ok(TelemetryField::GpsGroundSpeed),
            0x0E => Ok(TelemetryField::GpsHeading),
            0x0F => Ok(TelemetryField::Time),
            _ => Err(ProtocolError::InvalidFieldId(value)),
        }
    }
}

pub fn process_telemetry(buf: &BytesMut) -> Result<TelemetryData, ProtocolError> {
    let mut data = TelemetryData::new();

    let mut remaining = buf.clone();
    while remaining.has_remaining() {
        let field_id = remaining.get_u8();
        match TelemetryField::try_from(field_id) {
            Ok(field) => {
                data.set_field(field, remaining.get_u16() as f32);
            },
            Err(err) => return Err(err),
        }
    }
    Ok(data)
}