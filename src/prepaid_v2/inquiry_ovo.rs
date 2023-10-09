use std::path::Path;
use super::ResponseStatus;
use serde::{Deserialize, Serialize};
use super::{ResponseCode, api_url};
use crate::{Error, username, api_key, sign_hash};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InquiryOvoReqBody {
    pub username: String,
    pub customer_id: String,
    pub sign: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InquiryOvoResponse {
    pub data: Option<InquiryOvoData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InquiryOvoData {
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub message: String,
    pub rc: ResponseCode,
}


// post request to check-balance
pub async fn inquiry_ovo(customer_id: String) -> Result<InquiryOvoData, Error> {
  let url = Path::new(api_url())
      .join("inquiry-ovo")
      .to_str()
      .unwrap()
      .to_owned();
  let client = reqwest::Client::new();

  let signature = sign_hash(format!("{}{}{}", username(), api_key(), customer_id).as_str());
  // send this with the intent to respond in json
  let res = client
      .post(&url)
      .header("Content-Type", "application/json")
      .body(
          serde_json::to_string(&InquiryOvoReqBody {
              username: username(),
              customer_id: customer_id,
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
  let body = res.text().await.map_err(|e| Error::ResponseError(e.to_string()))?;
  let result: InquiryOvoResponse = serde_json::from_str(&body).map_err(|e| Error::DeserializationError(e.to_string()))?;
  println!("result: {:?}", result);
  Ok(result.data.unwrap())
}