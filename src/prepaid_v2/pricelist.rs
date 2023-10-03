use std::path::Path;
use serde::{Deserialize, Serialize};
use super::{ResponseCode, api_url, ProductType, ProductCategory};
use crate::{Error, username, api_key, sign_hash};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PricelistResponse {
    pub data: Option<PricelistData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PricelistData {
    pub pricelist: Option<Vec<Product>>,
    pub rc: ResponseCode,
    pub message: String,
    pub status: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub product_code: String,
    pub product_description: String,
    pub product_nominal: String,
    pub product_details: String,
    pub product_price: i64,
    pub product_type: Option<ProductType>,
    pub active_period: String,
    pub status: String,
    pub icon_url: String,
    pub product_category: Option<ProductCategory>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PricelistReqBody {
    pub username: String,
    pub sign: String,
    pub status: String,
}

// post request to get the pricelist.
// if product_code is supplied, it will filter based on the product_code
pub async fn pricelist(product_type_path: Option<String>, product_code: Option<String>) -> Result<PricelistData, Error> {
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
    let body = res.text().await.map_err(|e| Error::ResponseError(e.to_string()))?;;
    let result: PricelistResponse = serde_json::from_str(&body).map_err(|e| Error::ResponseError(e.to_string()))?;

    if let Some(product_code) = product_code {
        let data = result.data.clone().unwrap();
        let pricelist = data.pricelist.unwrap_or_default();
        let filtered = pricelist.into_iter().filter(|p| p.product_code == product_code).collect::<Vec<Product>>();
        return Ok(PricelistData {
            pricelist: Some(filtered),
            status: data.status,
            rc: data.rc,
            message: data.message,
        });
    }

    Ok(result.data.unwrap())
}
