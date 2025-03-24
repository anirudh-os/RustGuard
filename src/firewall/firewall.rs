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

    fn delete_rule_by_index(index: usize, firewall: &mut Firewall) {
        firewall.rules.remove(index);
    }

    pub fn delete_rule_by_protocol(protocol: Protocol, firewall: &mut Firewall) {
        firewall.rules.retain(|x| x.protocol != protocol);
    }

    pub fn delete_rule_by_source_ip(source_ip: &str, firewall: &mut Firewall) {
        firewall.rules.retain(|x| x.source_ip != source_ip);
    }

    pub fn delete_rule_by_destination_ip(destination_ip: &str, firewall: &mut Firewall) {
        firewall.rules.retain(|x| x.destination_ip != destination_ip);
    }

    pub fn delete_rule_by_port_range(starting_port_number: u32, ending_port_number: u32, firewall: &mut Firewall) {
        firewall.rules.retain(|x| x.starting_port_number != starting_port_number || x.ending_port_number != ending_port_number);
    }
}
