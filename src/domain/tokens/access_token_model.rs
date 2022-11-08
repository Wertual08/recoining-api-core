use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AccessTokenModel {
    pub sub: i64,                       // Optional. Subject (whom token refers to)
    pub exp: i64,                       // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: i64,                       // Optional. Issued at (as UTC timestamp)
    pub nbf: i64,                       // Optional. Not Before (as UTC timestamp)
    pub iss: String,                    // Optional. Issuer
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub aud: Option<String>,            // Optional. Audience
}