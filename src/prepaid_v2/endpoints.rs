use std::path::Path;
use crate::{Error, BalanceData, BalanceResponse, CheckBalanceReqBody};
use crate::{PROD_PREPAID_V2, DEV_PREPAID_V2, CheckStatusReqBody};


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





// post request to check-status of a transaction's ref_id
pub async fn check_status(ref_id: String) -> Result<BalanceData, Error> {
  let url = Path::new(api_url())
      .join("check-status")
      .to_str()
      .unwrap()
      .to_owned();
  let client = reqwest::Client::new();

  let signature = sign_hash(format!("{}{}bl", username(), api_key()).as_str());
  // send this with the intent to respond in json
  let res = client
      .post(&url)
      .header("Content-Type", "application/json")
      .body(
          serde_json::to_string(&CheckStatusReqBody {
              username: username(),
              ref_id: ref_id,
              sign: signature,
          })
          .unwrap(),
      )
      .send()
      .await
      .map_err(|e| Error::ResponseError(e.to_string()))?;

  if res.status() != 200 {
      return Err(Error::ResponseError(format!(
          "Response status code: {}",
          res.status()
      )));
  }
  let body = res.text().await.unwrap();
  let result: BalanceResponse = serde_json::from_str(&body).unwrap();
  println!("result: {:?}", result);
  Ok(result.data.unwrap())
}
