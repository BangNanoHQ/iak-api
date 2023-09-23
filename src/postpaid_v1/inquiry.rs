use super::{api_url, ResponseCode, ProductType, ProductStatus};
use crate::{Error, username, api_key, sign_hash};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct InquiryResponse {
    pub data: Option<InquiryData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InquiryData {
    pub tr_id: Option<i32>,
    pub code: Option<String>,
    pub datetime: Option<String>, // transaction time on BPJS

    // make it standardized like prepaid V2. 
    // this means that when this is shown to end users, it will show as customer_id, but it will parse it as hp
    #[serde(rename(deserialize = "hp"))]
    pub customer_id: Option<String>,
    
    pub tr_name: Option<String>, // bill account name
    pub period: Option<String>, // bill period
    pub nominal: Option<i32>, // bill nominal
    pub admin: Option<i32>, // admin fee
    pub ref_id: Option<Uuid>,
    pub response_code: Option<ResponseCode>,    
    pub message: Option<String>,
    pub price: Option<i32>, // Total price that must be paid (nominal + admin fee)
    pub selling_price: Option<i32>, // deducted balance. how much IAK charges.
    pub balance: Option<i32>, // on BPJS. Client remaining balance
    pub noref: Option<String>, // on BPJS Biller reference number (if exists)
    
    pub desc: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivInquiryReqBody {
    pub commands: String, 
    pub username: String,
    pub code: String,
    pub hp: String,
    pub ref_id: Uuid,
    pub sign: String,

    pub month: Option<String>, // only on BPJS AND BPJSTK NOT BPJSTKPU
    pub nomor_identitas: Option<String>, // only on ESAMSAT
    pub year: Option<String>, // only on PBB* - Number of year you're willing to pay
    pub desc: Option<CustomDenomEmoneyDesc>, // only on SHOPEEPAY or type emoney
}

impl From<InquiryReqBody> for PrivInquiryReqBody {
    fn from(body: InquiryReqBody) -> Self {
        PrivInquiryReqBody {
            commands: "inq-pasca".to_string(),
            username: username(),
            code: body.product_code,
            hp: body.customer_id,
            ref_id: body.ref_id.clone(),
            sign: sign_hash(format!("{}{}{}", username(), api_key(), body.ref_id).as_str()),
            month: body.month,
            nomor_identitas: body.identity_number,
            year: body.year,
            desc: body.desc,
        }
    }
}

// made the fields to to be similar to V2 on prepaid and made it to english
#[derive(Serialize, Deserialize, Debug)]
pub struct InquiryReqBody{  
    pub product_code: String,
    
    // on BPJS AND BPJSTK AND BPJSTKPU: participant number
    // on PGAS: customer number
    // on FNMEGA: contract number
    // on PDAM, AETRA: customer number
    // on PLNPOSTPAID: customer number
    // on PLNNONH: PLN customer number
    // on telkomvision TVTLKMV: customer number
    // on tv other than telkomvision TVBIG: customer number
    // on telco post paid HPTHREE: customer phone number
    // on Internet telkom speedy and pstn TELKOMPSTN: customer number
    // on CBN: customer number
    // on ESAMSAT: ESAMSAT payment code. To get it Download Samolnas apps, Reqister and get payment code
    // on type: "pbb", PBB*: Tax object number
    // on type: "emoney", SHOPEEPAY: Custom Denom payment code
    pub customer_id: String,
    pub ref_id: Uuid,
    pub month: Option<String>, // only on BPJS AND BPJSTK NOT BPJSTKPU
    pub identity_number: Option<String>, // only on ESAMSAT
    pub year: Option<String>, // only on PBB* - Number of year you're willing to pay
    pub desc: Option<CustomDenomEmoneyDesc>, // only on SHOPEEPAY or type emoney
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomDenomEmoneyDesc {
    pub amount: i32,
}

// post request to get the inquiry
pub async fn inquiry(req_body: InquiryReqBody) -> Result<InquiryData, Error> {
    let path: Vec<String> = vec!["bill/check".to_string()];

    let url = Path::new(api_url())
        .join(path.join("/"))
        .to_str()
        .unwrap()
        .to_owned();
    let client = reqwest::Client::new();

    let req_body: PrivInquiryReqBody = req_body.into();

    // send this with the intent to respond in json
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&req_body)
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
    let result: InquiryResponse = serde_json::from_str(&body).unwrap();
    Ok(result.data.unwrap())
}
