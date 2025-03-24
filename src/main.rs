mod firewall;

use firewall::firewall::Firewall;
use firewall::rule::{Rule, Action, Protocol};
use firewall::packet::Packet;

use clap::{arg, Command};
use serde_json;
use std::fs::File;
use std::io::{self, read_to_string, Write};
use serde::de::Error;

fn read_json_file(firewall: &mut Firewall, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let parsed: Vec<Rule> = serde_json::from_reader(file)?;
    firewall.rules.extend(parsed);
    Ok(())
}

fn write_json_file(firewall: Firewall, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    
}

fn main() {
    let matches = Command::new("Firewall Simulator")
        .version("1.0")
        .author("AlphaSlayer001")
        .about("This program simulates a basic firewall")
        .subcommand(
            Command::new("add_rule")
                .about("Add a new firewall rule")
                .arg(arg!(--action <ACTION> "Action to take: Allow or Deny"))
                .arg(arg!(--protocol <PROTOCOL> "Protocol: TCP or UDP"))
                .arg(arg!(--start <START> "Starting port number").value_parser(clap::value_parser!(u16)))
                .arg(arg!(--end <END> "Ending port number").value_parser(clap::value_parser!(u16)))
                .arg(arg!(--source <SOURCE> "Source IP address"))
                .arg(arg!(--destination <DESTINATION> "Destination IP address"))
        )
        .subcommand(
            Command::new("list_rules")
                .about("List all firewall rules")
        )
        .subcommand(
            Command::new("simulate_traffic")
                .about("Simulate network traffic")
                .arg(arg!(--protocol <PROTOCOL> "Protocol: TCP or UDP"))
                .arg(arg!(--port <PORT> "Port number").value_parser(clap::value_parser!(u16)))
                .arg(arg!(--source <SOURCE> "Source IP address"))
                .arg(arg!(--destination <DESTINATION> "Destination IP address"))
        )
        .get_matches();

    let mut firewall = Firewall::new();
    match read_json_file(&mut firewall, "rules.json") {
        Err(e) => println!("An error {} occurred!", e),
        _  => {}
    }

    if let Some(matches) = matches.subcommand_matches("add_rule") {
        let action_str = matches.get_one::<String>("action").unwrap();
        let action = match action_str.to_lowercase().as_str() {
            "allow" => Action::Allow,
            "deny" => Action::Deny,
            _ => panic!("Invalid action: must be 'Allow' or 'Deny'"),
        };

        let protocol_str = matches.get_one::<String>("protocol").unwrap();
        let protocol = match protocol_str.to_lowercase().as_str() {
            "tcp" => Protocol::TCP,
            "udp" => Protocol::UDP,
            _ => panic!("Invalid protocol: must be 'TCP' or 'UDP'"),
        };

        let starting_port_number = *matches.get_one::<u16>("start").unwrap() as u32;
        let ending_port_number = *matches.get_one::<u16>("end").unwrap() as u32;
        let source_ip = matches.get_one::<String>("source").unwrap().to_string();
        let destination_ip = matches.get_one::<String>("destination").unwrap().to_string();

        let rule = Rule {
            action,
            protocol,
            starting_port_number,
            ending_port_number,
            source_ip,
            destination_ip,
        };

        firewall.add_rule(rule);
        println!("Rule added successfully.");
    }

    if let Some(_) = matches.subcommand_matches("list_rules") {
        firewall.list_rules();
    }

    if let Some(matches) = matches.subcommand_matches("simulate_traffic") {
        let protocol_str = matches.get_one::<String>("protocol").unwrap();
        let protocol = match protocol_str.to_lowercase().as_str() {
            "tcp" => Protocol::TCP,
            "udp" => Protocol::UDP,
            _ => panic!("Invalid protocol: must be 'TCP' or 'UDP'"),
        };

        let port = *matches.get_one::<u32>("port").unwrap();
        let source_ip = matches.get_one::<String>("source").unwrap().to_string();
        let destination_ip = matches.get_one::<String>("destination").unwrap().to_string();

        let packet = Packet {
            protocol,
            port,
            source_ip,
            destination_ip,
        };

        let result = firewall.filter(packet);
        println!("{}", if result { "Packet allowed." } else { "Packet denied." });
    }
}