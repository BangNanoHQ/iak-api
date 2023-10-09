use super::{api_url, ResponseCode, ProductType, ProductStatus, PaymentData};
use crate::{Error, username, api_key, sign_hash};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckStatusResponse {
    pub data: Option<PaymentData>,
}




// made the fields to to be similar to V2 on prepaid and made it to english
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckStatusReqBody{  
    pub commands: String,
    pub ref_id: Uuid,
    pub username: String,
    pub sign: String,
}


// post request to get the check_status
pub async fn check_status(ref_id: Uuid) -> Result<PaymentData, Error> {
    let path: Vec<String> = vec!["bill/check".to_string()];

    let url = Path::new(api_url())
        .join(path.join("/"))
        .to_str()
        .unwrap()
        .to_owned();
    let client = reqwest::Client::new();

    let signature = sign_hash(format!("{}{}cs", username(), api_key()).as_str());

    // send this with the intent to respond in json
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&CheckStatusReqBody{
              commands: "checkstatus".to_string(),
              ref_id: ref_id,
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
    let result: CheckStatusResponse = serde_json::from_str(&body).map_err(|e| Error::DeserializationError(e.to_string()))?;
    Ok(result.data.unwrap())
}
