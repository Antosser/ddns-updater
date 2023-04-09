# DDNS Updater
Simple application for updating ddns on Google servers

## How to use
1. Download the binary from the lastest releases page.
2. Run it while providing the path to the toml config file as an argument

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
