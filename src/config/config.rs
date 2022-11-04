use std::{env, fs::File, error::Error};

use serde::{Deserialize, Serialize};

use crate::{storage::ScyllaConfig, domain::ServicesConfig};

use super::ServerConfig;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub scylla: ScyllaConfig,
    pub services: ServicesConfig,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();

        let mut config_path = None;
        
        let mut i = 0;
        while let Some(arg) = args.get(i) {
            if arg == "-config" {
                if let Some(path) = args.get(i + 1) {
                    config_path = Some(path)
                }
                else {
                    panic!("Path for -config must be specified")
                }
            }
            
            i += 1;   
        }

        let result = if let Some(path) = config_path {
            serde_json::from_reader(File::open(path)?)?
        }
        else {
            if let Ok(file) = File::open("config.json") {
                serde_json::from_reader(file)?
            }
            else {
                panic!("Config not found")
            }
        };

        Ok(result)
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}