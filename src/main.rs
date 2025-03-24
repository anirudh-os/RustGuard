mod firewall;

use firewall::firewall::Firewall;
use firewall::rule::{Rule, Action, Protocol};
use firewall::packet::Packet;

use clap::{arg, Command};
use serde_json;
use std::fs::File;

fn read_json_file(firewall: &mut Firewall, filepath: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    match serde_json::from_reader::<_, Vec<Rule>>(file) {
        Ok(parsed) => {
            let is_empty = parsed.is_empty();
            firewall.rules.extend(parsed);
            Ok(is_empty)
        }
        Err(e) => {
            if e.is_eof() {
                Ok(true) // treat as empty
            } else {
                Err(Box::new(e))
            }
        }
    }
}

fn write_json_file(firewall: &Firewall, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filepath)?;
    serde_json::to_writer_pretty(file, &firewall.rules)?;
    Ok(())
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
                .arg(arg!(--port <PORT> "Port number").value_parser(clap::value_parser!(u32)))
                .arg(arg!(--source <SOURCE> "Source IP address"))
                .arg(arg!(--destination <DESTINATION> "Destination IP address"))
        )
        .subcommand(
        Command::new("delete_rule")
            .about("Delete a firewall rule")
            .subcommand(
                Command::new("index")
                    .arg(arg!(<INDEX> "Index of the rule to delete").value_parser(clap::value_parser!(usize)))
            )
            .subcommand(
                Command::new("protocol")
                    .arg(arg!(<PROTOCOL> "Protocol: TCP or UDP"))
            )
            .subcommand(
                Command::new("source_ip")
                    .arg(arg!(<IP> "Source IP to delete"))
            )
            .subcommand(
                Command::new("destination_ip")
                    .arg(arg!(<IP> "Destination IP to delete"))
            )
            .subcommand(
                Command::new("port_range")
                    .arg(arg!(<START> "Start port").value_parser(clap::value_parser!(u32)))
                    .arg(arg!(<END> "End port").value_parser(clap::value_parser!(u32)))
            )
    ).get_matches();

    let mut firewall = Firewall::new();
    match read_json_file(&mut firewall, "src/rules.json") {
        Ok(empty) => {
            if empty && matches.subcommand_name() != Some("add_rule") {
                println!("rules.json is empty. Please add rules.");
            }
        },
        Err(e) => println!("An error {:#?} occurred!", e),
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

    if let Some(delete_matches) = matches.subcommand_matches("delete_rule") {
        match delete_matches.subcommand() {
            Some(("index", sub_m)) => {
                let index = *sub_m.get_one::<usize>("INDEX").unwrap();
                Firewall::delete_rule_by_index(index, &mut firewall);
            }
            Some(("protocol", sub_m)) => {
                let proto = sub_m.get_one::<String>("PROTOCOL").unwrap().to_lowercase();
                let protocol = match proto.as_str() {
                    "tcp" => Protocol::TCP,
                    "udp" => Protocol::UDP,
                    _ => {
                        println!("Invalid protocol.");
                        return;
                    }
                };
                Firewall::delete_rule_by_protocol(protocol, &mut firewall);
            }
            Some(("source_ip", sub_m)) => {
                let ip = sub_m.get_one::<String>("IP").unwrap();
                Firewall::delete_rule_by_source_ip(ip, &mut firewall);
            }
            Some(("destination_ip", sub_m)) => {
                let ip = sub_m.get_one::<String>("IP").unwrap();
                Firewall::delete_rule_by_destination_ip(ip, &mut firewall);
            }
            Some(("port_range", sub_m)) => {
                let start = *sub_m.get_one::<u32>("START").unwrap();
                let end = *sub_m.get_one::<u32>("END").unwrap();
                Firewall::delete_rule_by_port_range(start, end, &mut firewall);
            }
            _ => println!("Invalid delete command."),
        }
    }

    if let Err(e) = write_json_file(&firewall, "src/rules.json") {
        println!("An error {} occurred!", e);
    }
}