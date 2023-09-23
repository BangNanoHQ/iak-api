use super::{PROD_POSTPAID_V1, DEV_POSTPAID_V1};


// function to get the endpoint based on env DEV or PROD
pub fn api_url() -> &'static str {
  match std::env::var("IAK_API_ENV") {
      Ok(val) => {
          if val == "PROD" {
              PROD_POSTPAID_V1
          } else {
              DEV_POSTPAID_V1
          }
      }
      Err(_e) => panic!("IAK_API_ENV is not set"),
  }
}