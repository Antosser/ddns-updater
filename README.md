# DDNS Updater
Simple application for updating ddns on Google servers

## Usage
```
Application for updating ddns on Google servers

Usage: ddns-updater [OPTIONS]

Options:
  -c, --config-file <CONFIG_FILE>  The location of the .toml config file [default: ddns.toml]
  -h, --help                       Print help
```

## TOML file
The program requires a TOML config file to work. It must either be called `ddns.toml` in the wd or be provided as an argument

### TOML config structure
```toml
username = "myusername"
password = "mypassword"
hostname = "antonaparin.com"
```

## Compiling yourself - For linux users
1. Install Rust
2. Run `cargo build -r` in the project root`
3. Binary is in `target/release/`
