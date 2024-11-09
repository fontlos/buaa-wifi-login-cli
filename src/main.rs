use buaa_api::Session;
use clap::Parser;
use serde::{Deserialize, Serialize};

use std::fs::{File, OpenOptions};

#[derive(Debug, Parser)]
#[command(
    version = "0.1.0",
    about = "A cli to login to BUAA WiFi",
)]
struct Arg {
    #[arg(short, long)]
    username: Option<String>,
    #[arg(short, long)]
    password: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Config {
    username: String,
    password: String,
}

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("buaa-wifi-login-cli-config.json")
        .unwrap();
    let mut config = match serde_json::from_reader::<File, Config>(file){
        Ok(config) => config,
        Err(_) => Config::default(),
    };
    let arg = Arg::parse();
    if let Some(username) = arg.username {
        config.username = username;
    }
    if let Some(password) = arg.password {
        config.password = password;
    }
    let file = OpenOptions::new()
        .write(true)
        .open("buaa-wifi-login-cli-config.json")
        .unwrap();
    serde_json::to_writer(file, &config).unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let session = Session::new_in_memory();

    runtime.block_on(async {
        match session.gw_login(&config.username, &config.password).await {
            Ok(_) => println!("[Info]: Login successfully, Please wait a few seconds for the server to respond"),
            Err(e) => eprintln!("[Info]: Login failed: {:?}", e),
        }
    });
}
