use crate::firewall::rule::{Rule, Action};
use crate::firewall::packet::Packet;

#[derive(Debug)]
pub struct Firewall {
    pub rules: Vec<Rule>,
}

impl Firewall {
    pub fn new() -> Self {
        Firewall { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn list_rules(&self) {
        for (i, rule) in self.rules.iter().enumerate() {
            println!("Rule {}:\n{:#?}", i, rule);
        }
    }

    pub fn filter(&self, packet: Packet) -> bool {
        for rule in &self.rules {
            match rule.action {
                Action::Allow => {
                    if packet.protocol == rule.protocol
                        && packet.port >= rule.starting_port_number
                        && packet.port <= rule.ending_port_number
                        && packet.source_ip == rule.source_ip
                        && packet.destination_ip == rule.destination_ip
                    {
                        return true;
                    }
                }
                Action::Deny => {
                    println!(
                        "Packet {:#?} denied by the special rule {:#?}",
                        packet, rule
                    );
                }
            }
        }
        false
    }

    // Deletion methods can go here (optional)
}
