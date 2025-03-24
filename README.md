# RustGuard

A simple command-line based firewall simulator written in Rust.
You can add, list, delete rules, and simulate network traffic to test against the rules.

---

## Build

To compile the project in debug mode:

```bash
cargo build
```

## Run

Run the CLI tool using `cargo run` or the generated binary (after building):

```bash
cargo run -- <COMMAND> [OPTIONS]
```

Or, after building:

```bash
./target/debug/Firewall-Simulator <COMMAND> [OPTIONS]
```

---

## Available Commands

### Add a Rule

```bash
cargo run -- add_rule --action Allow --protocol TCP --start 1000 --end 2000 --source 192.168.1.1 --destination 8.8.8.8
```

### List All Rules

```bash
cargo run list_rules
```

### Simulate Traffic

```bash
cargo run simulate_traffic --protocol TCP --port 1123 --source 192.168.1.2 --destination 8.8.8.8
```

### Delete Rules

```bash
# Delete by index
cargo run -- delete_rule --by index --value 0

# Delete by protocol (TCP or UDP)
cargo run -- delete_rule --by protocol --value TCP

# Delete by source IP address
cargo run -- delete_rule --by source_ip --value 192.168.1.1

# Delete by destination IP address
cargo run -- delete_rule --by destination_ip --value 8.8.8.8

# Delete by port range
cargo run -- delete_rule --by port_range --start 1000 --end 2000
```
