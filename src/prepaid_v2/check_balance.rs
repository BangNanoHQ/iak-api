use std::path::Path;
use crate::Error;
use serde::{Deserialize, Serialize};
use crate::{ResponseCode, username, api_key, sign_hash, api_url};

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckBalanceReqBody {
    pub username: String,
    pub sign: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckStatusReqBody {
    pub username: String,
    pub ref_id: String,
    pub sign: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResponse {
    pub data: Option<BalanceData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceData {
    pub balance: Option<u32>,
    pub rc: ResponseCode,
    pub message: String,
    pub status: Option<u32>,
}


// post request to check-balance
pub async fn check_balance() -> Result<BalanceData, Error> {
  let url = Path::new(api_url())
      .join("check-balance")
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
          serde_json::to_string(&CheckBalanceReqBody {
              username: username(),
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