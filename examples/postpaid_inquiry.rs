use std::{fmt::Error, str::FromStr};

use iak_api::postpaid_v1::{self, InquiryReqBody};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let req_body = InquiryReqBody{
    product_code: "SHOPEEPAY".to_string(),
    customer_id: "082100000001".to_string(),
    // ref_id: Uuid::from_str("76da9e12-0cd7-52ca-a297-feabd383ca5c").unwrap(),
    ref_id: Uuid::new_v4(),
    month: None,
    identity_number: None,
    year: None,
    desc: Some(postpaid_v1::CustomDenomEmoneyDesc{
      amount: 10000,
    })
  };
  let products = postpaid_v1::inquiry(req_body).await.unwrap();
  println!("inquiry {:?}", products );

  // print products in json
  println!("inquiry JSON {}", serde_json::to_string(&products).unwrap() );
  


  Ok(())
}