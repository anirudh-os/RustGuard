use super::rule::Protocol;

#[derive(Debug)]
pub struct Packet {
    pub protocol: Protocol,
    pub port: u32,
    pub source_ip: String,
    pub destination_ip: String,
}
