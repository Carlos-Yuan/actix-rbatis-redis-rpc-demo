use std::{fs::{self, File}, path::Path, process::exit,env};

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub redis: Vec<String>,
    pub database: Database
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: String::from("127.0.0.1"),
            port: 8080,
            redis:vec!["redis://127.0.0.1:6379".to_string()],
            database: Database {
                host: String::from("127.0.0.1"),
                port: 3306,
                database: String::from("rust"),
                username: String::from("root"),
                password: String::from("123456")
            }
        }
    }
}

impl Config {
    pub fn rpcx_addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl Database {
    pub fn assemble(&self) -> String {
        format!("mysql://{}:{}@{}:{}/{}", self.username, self.password, self.host, self.port, self.database)
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