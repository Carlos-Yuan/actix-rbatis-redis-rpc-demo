use std::{fs::{self, File}, path::Path, process::exit,env};

use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub secret: String,
    pub prefix: String,
    pub redis: Vec<String>,
    pub rpc: Rpc
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Rpc {
    pub host: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: String::from("127.0.0.1"),
            port: 8080,
            secret: Utc::now().timestamp_nanos().to_string(),
            prefix:"api/v1".into(),
            redis:vec!["redis://127.0.0.1:6379".to_string()],
            rpc: Rpc {
                host: String::from("127.0.0.1"),
                port: 8888,
            }
        }
    }
}

impl Rpc {
    pub fn address(&self) -> String {
        format!("{}:{}",self.host, self.port)
    }
}

pub fn load() -> Config {
    let args:Vec<String>=env::args().collect();
    if args.len()!=2{
        panic!("no config file!")
    }
    let config_path = Path::new(args[1].as_str());
    if !config_path.exists() {
        log::error!("config file not found, creating one and aborting process");
        File::create(config_path).expect("failed to create config file");
        fs::write(config_path, toml::to_string_pretty(&Config::default()).expect("error while trying to create new config")).expect("error while trying to generate default config specs");
        exit(-1)
    }
    let config: Config = toml::from_str(fs::read_to_string(config_path).expect("error while reading the config file").as_str()).expect("failed to parse config");
    config
}