use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ScyllaConfig {
    pub hosts: Vec<String>,
    pub keyspace: String,
}