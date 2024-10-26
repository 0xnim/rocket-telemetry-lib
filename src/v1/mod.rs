pub mod protocol;
pub mod telemetry;
pub mod builder;
pub mod packet;
mod types;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    use crate::common::function::vec_to_bytes_mut;
    #[test]
    fn test_telemetry() {
        let mut file = File::open(Path::new("test_data/v1/telemetry.bin")).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        data.drain(..8);
        let telemetry_data = super::telemetry::process_telemetry(&vec_to_bytes_mut(data)).unwrap();
        println!("{:?}", telemetry_data);
    }

    use crate::v1::types::TelemetryData;
    use crate::v1::packet::PacketPayload;
    use crate::common::types::MessageType;
    use crate::v1::packet::V1Packet;
    #[test]
    fn test_builder() {
        let telemetry_data = TelemetryData::new();
        let payload = PacketPayload::new(telemetry_data);
        let packet = V1Packet::new(MessageType::Telemetry, 0, 0, payload);
        println!("{:?}", packet);
        println!("{:?}", packet.to_binary());

    }
}