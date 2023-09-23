use std::path::Path;
use serde::{Deserialize, Serialize};
use super::{ResponseCode, api_url, ProductType};
use crate::{Error, username, api_key, sign_hash};



#[derive(Serialize, Deserialize, Debug)]
pub struct PricelistResponse {
    pub data: Option<PricelistData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PricelistData {
    pub pricelist: Option<Vec<Product>>,
    pub rc: ResponseCode,
    pub message: String,
    pub status: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub product_code: String,
    pub product_description: String,
    pub product_nominal: String,
    pub product_details: String,
    pub product_price: u32,
    pub product_type: ProductType,
    pub active_period: String,
    pub status: String,
    pub icon_url: String,
    pub product_category: ProductType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PricelistReqBody {
    pub username: String,
    pub sign: String,
    pub status: String,
}

// post request to get the pricelist
pub async fn pricelist(product_type_path: Option<String>) -> Result<PricelistData, Error> {
    let mut path: Vec<String> = vec!["pricelist".to_string()];

    if let Some(product_type_path) = product_type_path {
        path.push(product_type_path.to_string());
    }

    let url = Path::new(api_url())
        .join(path.join("/"))
        .to_str()
        .unwrap()
        .to_owned();
    let client = reqwest::Client::new();

    let signature = sign_hash(format!("{}{}pl", username(), api_key()).as_str());
    // send this with the intent to respond in json
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&PricelistReqBody {
                username: username(),
                sign: signature,
                status: "all".to_string(),
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
    let result: PricelistResponse = serde_json::from_str(&body).unwrap();
    Ok(result.data.unwrap())
}
