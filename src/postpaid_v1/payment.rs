use super::{api_url, ResponseCode, ProductType, ProductStatus};
use crate::{Error, username, api_key, sign_hash};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentResponse {
    pub data: Option<PaymentData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentData {
    pub tr_id: Option<i64>,
    pub code: Option<String>,
    pub datetime: Option<String>, // transaction time on BPJS

    // make it standardized like prepaid V2. 
    // this means that when this is shown to end users, it will show as customer_id, but it will parse it as hp
    #[serde(rename(deserialize = "hp"))]
    pub customer_id: Option<String>,
    
    pub tr_name: Option<String>, // bill account name
    pub period: Option<String>, // bill period
    pub nominal: Option<i64>, // bill nominal
    pub admin: Option<i64>, // admin fee
    pub ref_id: Option<Uuid>,
    pub response_code: Option<ResponseCode>,    
    pub message: Option<String>,
    pub price: Option<i64>, // Total price that must be paid (nominal + admin fee)
    pub selling_price: Option<i64>, // deducted balance. how much IAK charges.
    pub balance: Option<i64>, // on BPJS. Client remaining balance in IAK
    pub noref: Option<String>, // on BPJS Biller reference number (if exists)
    
    pub desc: Option<serde_json::Value>,
}


// made the fields to to be similar to V2 on prepaid and made it to english
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentReqBody{  
    pub commands: String,
    pub tr_id: u32,
    pub username: String,
    pub sign: String,
}


// post request to get the payment
pub async fn payment(tr_id: u32) -> Result<PaymentData, Error> {
    let path: Vec<String> = vec!["bill/check".to_string()];

    let url = Path::new(api_url())
        .join(path.join("/"))
        .to_str()
        .unwrap()
        .to_owned();
    let client = reqwest::Client::new();

    let signature = sign_hash(format!("{}{}{}", username(), api_key(), tr_id).as_str());

    // send this with the intent to respond in json
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&PaymentReqBody{
              commands: "pay-pasca".to_string(),
              tr_id: tr_id,
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
    println!("original resp body: {}", body);
    let result: PaymentResponse = serde_json::from_str(&body).unwrap();
    Ok(result.data.unwrap())
}
