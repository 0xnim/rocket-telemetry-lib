// src/lib.rs
mod error;
mod packet;
mod v1;
mod v2;
mod common;

use error::ProtocolError;
pub use packet::Packet;

/// Processes a packet based on its version
pub fn process_packet(data: &[u8]) -> Result<Packet, ProtocolError> {
    if data.len() <= 8 {
        return Err(ProtocolError::InvalidLength);
    }

    let version = data[0];
    match version {
        1 => v1::protocol::process_packet(data),
        //2 => v2::protocol::process_packet(data),
        _ => Err(ProtocolError::UnsupportedVersion(version)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    // console log the binary data
    /*
    #[test]
    fn debug_test_packet() {
        let mut file = File::open(Path::new("test_data/test.bin")).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        println!("{:?}", data);
        // to binary from u8
        //println!("{}", data);

        // do example data proccessing manually
        // split the data into version, type, id, len, time, data
        /*let version = data[0..8].to_string();
        let type_ = data[8..10].to_string();
        let id = data[10..16].to_string();
        let len = data[16..32].to_string();
        let time = data[32..64].to_string();
        let data = data[64..].to_string();*/

        let version = data[0] & 0b01111111;
        let type_ = data[1] & 0b11000000; // First 2 bits
        let message_type = crate::common::types::MessageType::try_from(type_).unwrap();
        let id = data[1] & 0b00111111; // Last 6 bits
        let len = u16::from_be_bytes([data[2], data[3]]);
        let time = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        let data = data[8..].to_vec();


        println!("Version: {}", version);
        println!("Type: {}", type_);
        println!("Message Type: {:?}", message_type);
        println!("ID: {}", id);
        println!("Len: {}", len);
        println!("Time: {}", time);
    }*/

    #[test]
    fn test_process_packet() {
        let mut file = File::open(Path::new("test_data/v1/telemetry.bin")).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let packet = process_packet(&data).unwrap();
        println!("{:?}", packet);
    }
}
