use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq)]
pub enum Protocol {
    TCP,
    UDP,
    #[allow(dead_code)]
    All,
}

#[derive(Debug)]
pub enum Action {
    Allow,
    Deny,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Rule {
    pub action: Action,
    pub protocol: Protocol,
    pub starting_port_number: u32,
    pub ending_port_number: u32,
    pub source_ip: String,
    pub destination_ip: String,
}
