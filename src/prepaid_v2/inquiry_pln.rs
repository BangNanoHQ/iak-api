use std::path::Path;
use super::ResponseStatus;
use serde::{Deserialize, Serialize};
use super::{ResponseCode, api_url};
use crate::{Error, username, api_key, sign_hash};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InquiryPlnReqBody {
    pub username: String,
    pub customer_id: String,
    pub sign: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InquiryPlnResponse {
    pub data: Option<InquiryPlnData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InquiryPlnData {
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_power: Option<String>,
    pub message: String,
    pub rc: ResponseCode,
}


// post request to check-balance
pub async fn inquiry_pln(customer_id: String) -> Result<InquiryPlnData, Error> {
  let url = Path::new(api_url())
      .join("inquiry-pln")
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
          serde_json::to_string(&InquiryPlnReqBody {
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
  let body = res.text().await.unwrap();
  let result: InquiryPlnResponse = serde_json::from_str(&body).unwrap();
  println!("result: {:?}", result);
  Ok(result.data.unwrap())
}