use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Response error: `{0}`")]
    ResponseError(String),

    #[error("Decryption error: `{0}`")]
    DecryptionError(String),

    #[error("Deserialization error: `{0}`")]
    DeserializationError(String),

    #[error("unknown model error")]
    Unknown,
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
