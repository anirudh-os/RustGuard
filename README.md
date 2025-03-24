# RustGuard

A simple command-line based firewall simulator written in Rust.
You can add, list, delete rules, and simulate network traffic to test against the rules.

---

## Features

- Add new firewall rules
- View current rules in a readable format
- Delete rules by various criteria
- Simulate network traffic to check against rules
- Persistent rule storage using JSON

---

## Requirements

- Rust 
- `serde`, `serde_json`, and `clap` crates

## Installation

- Make sure you have Rust installed.
- Clone the repository and build the project:
```bash
git clone https://github.com/anirudh-os/RustGuard.git
cd RustGuard
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

Contributing
------------

Contributions are welcome! If you'd like to improve this project, follow these steps:

1.  Fork the repository.

2.  Create a new branch (`git checkout -b feature-name`).

3.  Make your changes.

4.  Commit your changes (`git commit -m 'Add feature'`).

5.  Push to your fork (`git push origin feature-name`).

6.  Open a pull request.

Make sure your code is formatted (`cargo fmt`) and passes clippy checks (`cargo clippy`) before submitting.
