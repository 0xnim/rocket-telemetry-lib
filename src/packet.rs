// src/packet.rs

use crate::v1::packet::V1Packet;

#[derive(Debug)]
pub enum Packet {
    V1(V1Packet),
    //V2(V2Packet),
}
