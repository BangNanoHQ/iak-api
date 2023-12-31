use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Error, Debug)]
pub enum Error {
    #[error("Response error: `{0}`")]
    ResponseError(String),

    #[error("Decryption error: `{0}`")]
    DecryptionError(String),

    #[error("Deserialization error: `{0}`")]
    DeserializationError(String),

    #[error("Serialization error: `{0}`")]
    SerializationError(String),

    #[error("unknown model error")]
    Unknown,
}

// generate signature data username + api_key + any text
pub fn generate_iak_signature(text: String) -> String {
    let username = username();
    let api_key = api_key();
    sign_hash(&format!("{}{}{}", username, api_key, text))
}
  

pub fn username() -> String {
    match std::env::var("IAK_API_USERNAME") {
        Ok(val) => val,
        Err(_e) => panic!("IAK_API_USERNAME is not set"),
    }
}
pub fn api_key() -> String {
    match std::env::var("IAK_API_KEY") {
        Ok(val) => val,
        Err(_e) => panic!("IAK_API_KEY is not set"),
    }
}

pub fn sign_hash(text: &str) -> String {
    format!("{:x}", md5::compute(text))
}
