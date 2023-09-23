use std::{fmt::Error, str::FromStr};

use iak_api::postpaid_v1::{self, InquiryReqBody};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let products = postpaid_v1::payment(10122686).await.unwrap();
  println!("payment {:?}", products );

  // print products in json
  println!("payment JSON {}", serde_json::to_string(&products).unwrap() );
  


  Ok(())
}