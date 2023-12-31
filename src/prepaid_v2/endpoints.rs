use super::{PROD_PREPAID_V2, DEV_PREPAID_V2};


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