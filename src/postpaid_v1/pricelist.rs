use super::{api_url, ResponseCode, ProductType, ProductStatus};
use crate::{Error, username, api_key, sign_hash};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PricelistResponse {
    pub data: Option<PricelistData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PricelistData {
    pub pasca: Option<Vec<Product>>,
    pub response_code: Option<ResponseCode>,
    pub message: Option<String>,
    pub status: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub code: String,
    pub name: String,
    pub status: ProductStatus,
    pub fee: i64,
    pub komisi: i64, 
    pub r#type: ProductType,
    pub category: String,
    pub province: Option<String>, // 34 provinces in indonesia, only for PDAM

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PricelistReqBody {
    pub commands: String, 
    pub username: String,
    pub sign: String,
    pub status: String,
}

// post request to get the pricelist
pub async fn pricelist(product_type_path: Option<String>, product_code: Option<String>) -> Result<PricelistData, Error> {
    let mut path: Vec<String> = vec!["bill/check".to_string()];

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
                commands: "pricelist-pasca".to_string(),
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
    let result: PricelistResponse = serde_json::from_str(&body).map_err(|e| Error::DeserializationError(e.to_string()))?;

    let data = result.data.clone().ok_or(Error::ResponseError("No Data".to_string()))?;

    if let Some(product_code) = product_code {
        let pricelist = data.pasca.unwrap_or_default();
        let filtered = pricelist.into_iter().filter(|p| p.code == product_code).collect::<Vec<Product>>();
        return Ok(PricelistData {
            pasca: Some(filtered),
            status: data.status,
            response_code: data.response_code,
            message: data.message,
        });
    }

    Ok(data)
}
