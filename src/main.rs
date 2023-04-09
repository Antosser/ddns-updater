use clap::Parser;
use std::{fs, process::exit};

use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    username: String,
    password: String,
    hostname: String,
}

/// Application for updating ddns on Google servers
#[derive(Parser)]
struct Args {
    /// The location of the .toml config file
    #[clap(short, long, default_value = "ddns.toml")]
    config_file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let filename = args.config_file;

    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error while reading file: {}", e);
            exit(1);
        }
    };
    let data: Data = match toml::from_str(content.as_str()) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error while parsing file: {}", e);
            exit(1);
        }
    };

    let mut prev_ip: Option<String> = None;

    loop {
        let ip = public_ip::addr()
            .await
            .unwrap_or_else(|| panic!("Can't get IP-Address"))
            .to_string();

        println!("IP-Address: {}", ip);

        if *prev_ip.as_ref().unwrap_or(&"".to_owned()) != ip {
            prev_ip = Some(ip.clone());

            println!("Response: {}", 'print: {
                if let Ok(response) = reqwest::get(format!(
                    "https://{}:{}@domains.google.com/nic/update?hostname={}&myip={}",
                    data.username, data.password, data.hostname, ip
                ))
                .await
                {
                    if let Ok(text) = response.text().await {
                        break 'print text;
                    }
                }

                "Failed".to_owned()
            });
        } else {
            println!("IP-Address didn't change");
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
