use std::{fmt::Error, str::FromStr};

use iak_api::postpaid_v1::{self, InquiryReqBody};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let ref_id = Uuid::from_str("c3c72f0f-d8a9-572a-bcc4-5409f37a947c").unwrap();
  
  let products = postpaid_v1::check_status(ref_id).await.unwrap();
  println!("check_status {:?}", products );

  // print products in json
  println!("check_status JSON {}", serde_json::to_string(&products).unwrap() );
  


  Ok(())
}