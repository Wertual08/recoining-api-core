use std::{env, fs::File};

use serde::{Deserialize, Serialize};

use crate::storage::ScyllaConfig;

use super::ServerConfig;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub scylla: ScyllaConfig,
}

impl Config {
    pub fn new() -> Self {
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

        if let Some(path) = config_path {
            serde_json::from_reader(File::open(path).unwrap()).unwrap()
        }
        else {
            if let Ok(file) = File::open("config.json") {
                serde_json::from_reader(file).unwrap()
            }
            else {
                panic!("Config not found")
            }
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}