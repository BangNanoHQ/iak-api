use crate::{PROD_PREPAID_V2, DEV_PREPAID_V2};


// function to get the endpoint based on env DEV or PROD
pub fn api_url() -> &'static str {
  match std::env::var("IAK_API_ENV") {
      Ok(val) => {
          if val == "PROD" {
              PROD_PREPAID_V2
          } else {
              DEV_PREPAID_V2
          }
      }
      Err(_e) => panic!("IAK_API_ENV is not set"),
  }
}

pub fn username() -> String {
  match std::env::var("USERNAME") {
      Ok(val) => val,
      Err(_e) => panic!("USERNAME is not set"),
  }
}
pub fn api_key() -> String {
  match std::env::var("API_KEY") {
      Ok(val) => val,
      Err(_e) => panic!("API_KEY is not set"),
  }
}

pub fn sign_hash(text: &str) -> String {
  format!("{:x}", md5::compute(text))
}





