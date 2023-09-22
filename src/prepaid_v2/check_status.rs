use std::path::Path;
use crate::{Error, ResponseStatus, TopupData, TopupResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{ResponseCode, username, api_key, sign_hash, api_url};

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckStatusReqBody {
    pub username: String,
    pub ref_id: String,
    pub sign: String,
}



// post request to check-balance
pub async fn check_status(ref_id: Uuid) -> Result<TopupData, Error> {
  let url = Path::new(api_url())
      .join("check-status")
      .to_str()
      .unwrap()
      .to_owned();
  let client = reqwest::Client::new();

  let signature = sign_hash(format!("{}{}{}", username(), api_key(), ref_id).as_str());
  // send this with the intent to respond in json
  let res = client
      .post(&url)
      .header("Content-Type", "application/json")
      .body(
          serde_json::to_string(&CheckStatusReqBody {
              username: username(),
              ref_id: ref_id.to_string(),
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
  let result: TopupResponse = serde_json::from_str(&body).unwrap();
  println!("result: {:?}", result);
  Ok(result.data.unwrap())
}