use crate::firewall::rule::{Rule, Action, Protocol};
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
            match serde_json::to_string_pretty(rule) {
                Ok(json) => println!("Rule {}:\n{}\n", i, json),
                Err(e) => eprintln!("Failed to serialize rule {}: {}\n", i, e),
            }
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

    pub fn delete_rule_by_index(index: usize, firewall: &mut Firewall) {
        if index < firewall.rules.len() {
            let removed = firewall.rules.remove(index);
            println!("Deleted rule at index {}:\n{:#?}", index, removed);
        } else {
            println!("Invalid index: {}", index);
        }
    }

    pub fn delete_rule_by_protocol(protocol: Protocol, firewall: &mut Firewall) {
        let deleted: Vec<_> = firewall.rules
            .iter()
            .cloned()
            .filter(|x| x.protocol == protocol)
            .collect();

        firewall.rules.retain(|x| x.protocol != protocol);
        if deleted.is_empty() {
            println!("No rules found with protocol: {:?}", protocol);
        } else {
            for rule in deleted {
                println!("Deleted rule:\n{:#?}", rule);
            }
        }
    }

    pub fn delete_rule_by_source_ip(source_ip: &str, firewall: &mut Firewall) {
        let deleted: Vec<_> = firewall.rules
            .iter()
            .cloned()
            .filter(|x| x.source_ip == source_ip)
            .collect();

        firewall.rules.retain(|x| x.source_ip != source_ip);
        if deleted.is_empty() {
            println!("No rules found with source IP: {}", source_ip);
        } else {
            for rule in deleted {
                println!("Deleted rule:\n{:#?}", rule);
            }
        }
    }

    pub fn delete_rule_by_destination_ip(destination_ip: &str, firewall: &mut Firewall) {
        let deleted: Vec<_> = firewall.rules
            .iter()
            .cloned()
            .filter(|x| x.destination_ip == destination_ip)
            .collect();

        firewall.rules.retain(|x| x.destination_ip != destination_ip);
        if deleted.is_empty() {
            println!("No rules found with destination IP: {}", destination_ip);
        } else {
            for rule in deleted {
                println!("Deleted rule:\n{:#?}", rule);
            }
        }
    }

    pub fn delete_rule_by_port_range(starting_port_number: u32, ending_port_number: u32, firewall: &mut Firewall) {
        let deleted: Vec<_> = firewall.rules
            .iter()
            .cloned()
            .filter(|x| x.starting_port_number == starting_port_number && x.ending_port_number == ending_port_number)
            .collect();

        firewall.rules.retain(|x| x.starting_port_number != starting_port_number || x.ending_port_number != ending_port_number);
        if deleted.is_empty() {
            println!("No rules found in port range: {} - {}", starting_port_number, ending_port_number);
        } else {
            for rule in deleted {
                println!("Deleted rule:\n{:#?}", rule);
            }
        }
    }
}
