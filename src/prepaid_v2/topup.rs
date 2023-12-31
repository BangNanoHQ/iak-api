use std::path::Path;
use super::ResponseStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::{ResponseCode, api_url};
use crate::{Error, username, api_key, sign_hash};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopupReqBody {
    pub username: String,
    pub ref_id: String,
    pub customer_id: String,
    pub product_code: String,
    pub sign: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopupResponse {
    pub data: Option<TopupData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopupData {
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,

    pub status: ResponseStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr_id: Option<i64>,
    
    pub rc: ResponseCode,

    // used if it is a callback
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
}

// convert from TopupCallbackData into TopupData
impl TryFrom<TopupCallbackData> for TopupData{
    type Error = Error;
    fn try_from(data: TopupCallbackData) -> Result<Self, Self::Error> {
        Ok(TopupData {
            ref_id: Some(data.ref_id),
            status: data.status,
            product_code: Some(data.product_code),
            customer_id: Some(data.customer_id),
            price: Some(data.price.parse::<i64>().unwrap()),
            message: data.message,
            sn: data.sn,
            pin: data.pin,
            balance: Some(data.balance.parse::<i64>().unwrap()),
            tr_id: Some(data.tr_id.parse::<i64>().unwrap()),
            rc: data.rc,
            sign: Some(data.sign),
        })
    }
} 

#[derive(Deserialize, Debug, Clone)]
pub struct TopupCallbackResponse {
    pub data: Option<TopupCallbackData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TopupCallbackData {
    pub ref_id: String,
    pub status: ResponseStatus,
    pub product_code: String,
    pub customer_id: String,
    pub price: String,
    pub message: String,
    pub sn: Option<String>,

    pub pin: Option<String>,
    pub balance: String,
    pub tr_id: String,    
    pub rc: ResponseCode,
    pub sign: String,
}




// post request to check-balance
pub async fn topup(product_code: String, ref_id: Uuid, customer_id: String) -> Result<TopupData, Error> {
  let url = Path::new(api_url())
      .join("top-up")
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
          serde_json::to_string(&TopupReqBody {
              username: username(),
              ref_id: ref_id.to_string(),
              customer_id: customer_id,
              product_code: product_code,
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
  let result: TopupResponse = serde_json::from_str(&body).map_err(|e| Error::DeserializationError(e.to_string()))?;
  println!("result: {:?}", result);
  Ok(result.data.unwrap())
}