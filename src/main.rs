//use std::cmp::PartialEq;
use clap::{App, Arg, SubCommand};

#[derive(Debug)]
enum Action {
    Allow,
    Deny
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Protocol {
    TCP,
    UDP,
    All
}

#[derive(Debug)]
struct Rule {
    action: Action,
    protocol: Protocol,
    starting_port_number: u32,
    ending_port_number: u32,
    source_ip: String,
    destination_ip: String
}

#[derive(Debug)]
struct Firewall {
    rules: Vec<Rule>
}

#[derive(Debug)]
struct Packet {
    protocol: Protocol,
    port: u32,
    source_ip: String,
    destination_ip: String
}

impl Firewall {
    fn new() -> Self {
        Firewall {
            rules: Vec::new()
        }
    }

    fn filter(packet: Packet, firewall: Firewall) -> bool {
        let mut pass = false;
        for rule in &(firewall.rules) {
            match rule.action {
                Action::Allow => {
                    if (packet.protocol == rule.protocol) && (packet.port >= rule.starting_port_number && packet.port <= rule.ending_port_number) && (packet.source_ip == rule.source_ip) && (packet.destination_ip == rule.destination_ip) {
                        pass = true;
                        break;
                    } else {
                        pass = false
                    }
                }
                Action::Deny => {
                    println!("Packet {:#?} denied by the special rule {:#?}", packet, rule);
                }
            }
        }

        return pass;
    }

    fn add_rule(rule: Rule, firewall: &mut Firewall) {
        firewall.rules.push(rule);
    }

    fn list_rules(firewall: &mut Firewall) {
        for i in 0..firewall.rules.len() {
            println!("Rule {}:\n{:#?}", i, firewall.rules[i]);
        }
    }

    fn delete_rule_by_index(index: usize, firewall: &mut Firewall) {
        firewall.rules.remove(index);
    }

    fn delete_rule_by_protocol(protocol: Protocol, firewall: &mut Firewall) {
        firewall.rules.retain(|x| x.protocol != protocol);
    }

    fn delete_rule_by_source_ip(source_ip: &str, firewall: &mut Firewall) {
        firewall.rules.retain(|x| x.source_ip != source_ip);
    }

    fn delete_rule_by_destination_ip(destination_ip: &str, firewall: &mut Firewall) {
        firewall.rules.retain(|x| x.destination_ip != destination_ip);
    }

    fn delete_rule_by_port_range(starting_port_number: u32, ending_port_number: u32, firewall: &mut Firewall) {
        firewall.rules.retain(|x| x.starting_port_number != starting_port_number || x.ending_port_number != ending_port_number);
    }

}

fn main() {
    let matches = App::new("Firewall Simulator")
        .version("1.0")
        .author("AlphaSlayer001")
        .about("This program simulates a basic firewall")
        .subcommand(
            SubCommand::with_name("add_rule")
                .about("Adds a new rule to the firewall")
                .arg(
                    Arg::with_name("action")
                        .help("Action to perform: allow or deny")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("protocol")
                        .help("Protocol: TCP, UDP, or All")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("starting_port")
                        .help("Starting port number")
                        .required(true)
                        .index(3),
                )
                .arg(
                    Arg::with_name("ending_port")
                        .help("Ending port number")
                        .required(true)
                        .index(4),
                )
                .arg(
                    Arg::with_name("source_ip")
                        .help("Source IP address")
                        .required(true)
                        .index(5),
                )
                .arg(
                    Arg::with_name("destination_ip")
                        .help("Destination IP address")
                        .required(true)
                        .index(6),
                ),
        )
        .subcommand(
            SubCommand::with_name("list_rules")
                .about("Lists all firewall rules"),
        )
        .subcommand(
            SubCommand::with_name("simulate_traffic")
                .about("Simulates a packet passing through the firewall")
                .arg(
                    Arg::with_name("protocol")
                        .help("Protocol of the packet: TCP, UDP, or All")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("port")
                        .help("Port number of the packet")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("source_ip")
                        .help("Source IP address of the packet")
                        .required(true)
                        .index(3),
                )
                .arg(
                    Arg::with_name("destination_ip")
                        .help("Destination IP address of the packet")
                        .required(true)
                        .index(4),
                ),
        )
        .get_matches();

    // Initialize the firewall
    let mut firewall = Firewall::new();

    // Handle "add_rule" command
    if let Some(matches) = matches.subcommand_matches("add_rule") {
        let action = match matches.value_of("action").unwrap() {
            "allow" => Action::Allow,
            "deny" => Action::Deny,
            _ => {
                eprintln!("Invalid action. Use 'allow' or 'deny'.");
                return;
            }
        };

        let protocol = match matches.value_of("protocol").unwrap() {
            "TCP" => Protocol::TCP,
            "UDP" => Protocol::UDP,
            "All" => Protocol::All,
            _ => {
                eprintln!("Invalid protocol. Use 'TCP', 'UDP', or 'All'.");
                return;
            }
        };

        let starting_port = matches.value_of("starting_port").unwrap().parse::<u32>().unwrap();
        let ending_port = matches.value_of("ending_port").unwrap().parse::<u32>().unwrap();
        let source_ip = matches.value_of("source_ip").unwrap().to_string();
        let destination_ip = matches.value_of("destination_ip").unwrap().to_string();

        let rule = Rule {
            action,
            protocol,
            starting_port_number: starting_port,
            ending_port_number: ending_port,
            source_ip,
            destination_ip,
        };

        Firewall::add_rule(rule, &mut firewall);
        println!("Rule added successfully.");
    }

    // Handle "list_rules" command
    if let Some(_) = matches.subcommand_matches("list_rules") {
        Firewall::list_rules(&mut firewall);
    }

    // Handle "simulate_traffic" command
    if let Some(matches) = matches.subcommand_matches("simulate_traffic") {
        let protocol = match matches.value_of("protocol").unwrap() {
            "TCP" => Protocol::TCP,
            "UDP" => Protocol::UDP,
            "All" => Protocol::All,
            _ => {
                eprintln!("Invalid protocol. Use 'TCP', 'UDP', or 'All'.");
                return;
            }
        };

        let port = matches.value_of("port").unwrap().parse::<u32>().unwrap();
        let source_ip = matches.value_of("source_ip").unwrap().to_string();
        let destination_ip = matches.value_of("destination_ip").unwrap().to_string();

        let packet = Packet {
            protocol,
            port,
            source_ip,
            destination_ip,
        };

        let result = Firewall::filter(packet, firewall);
        if result {
            println!("Packet allowed.");
        } else {
            println!("Packet denied.");
        }
    }
}
