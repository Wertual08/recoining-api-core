use serde::{Deserialize, Serialize};

use super::{tokens::TokensConfig, codes::CodesConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub codes: CodesConfig,
    pub tokens: TokensConfig,
}