use clap::Parser;
use log::{debug, error, info, warn};
use std::{fs, process::exit};

use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    username: String,
    password: String,
    hostname: String,
    ipv6: bool,
    timeout: u64,
}

/// Simple application for updating ddns on Google servers
#[derive(Parser)]
struct Args {
    /// The location of the .toml config file
    #[clap(short, long, default_value = "ddns.toml")]
    config_file: String,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    let filename = args.config_file;

    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            error!("Error while reading file: {}", e);
            exit(1);
        }
    };
    let data: Data = match toml::from_str(content.as_str()) {
        Ok(d) => d,
        Err(e) => {
            error!("Error while parsing file: {}", e);
            exit(1);
        }
    };

    let mut prev_ip: Option<String> = None;

    loop {
        let ip = {
            if data.ipv6 {
                // reqwest::get("http://checkip6.spdyn.de/")
                //     .await
                //     .unwrap()
                //     .text()
                //     .await
                //     .unwrap()
                match ureq::get("http://checkip6.spdyn.de/").call() {
                    Ok(response) => match response.into_string() {
                        Ok(text) => text,
                        Err(e) => {
                            warn!("Can't get IP-Address. Retrying in 10 seconds: {}", e);
                            std::thread::sleep(std::time::Duration::from_secs(2));

                            continue;
                        }
                    },
                    Err(e) => {
                        warn!("Can't get IP-Address. Retrying in 10 seconds: {}", e);
                        std::thread::sleep(std::time::Duration::from_secs(2));

                        continue;
                    }
                }
            } else {
                match public_ip::addr().await {
                    Some(x) => x.to_string(),
                    None => {
                        warn!("Can't get IP-Address. Retrying in 10 seconds");
                        std::thread::sleep(std::time::Duration::from_secs(2));

                        continue;
                    }
                }
            }
        };

        debug!("IP-Address: {}", ip);

        if *prev_ip.as_ref().unwrap_or(&"".to_owned()) != ip {
            prev_ip = Some(ip.clone());
            info!("IP-Address changed to {}", ip);

            info!("Response: {}", 'print: {
                if let Ok(response) = ureq::get(&format!(
                    "https://{}:{}@domains.google.com/nic/update?hostname={}&myip={}",
                    data.username, data.password, data.hostname, ip
                ))
                .call()
                {
                    if let Ok(text) = response.into_string() {
                        break 'print text;
                    }
                }

                "Failed".to_owned()
            });
        } else {
            debug!("IP-Address didn't change");
        }

        std::thread::sleep(std::time::Duration::from_secs(data.timeout));
    }
}
