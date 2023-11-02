use std::fmt::Error;

use iak_api::postpaid_v1;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let product_type_path = "".to_string();
  let product_code: Option<String> = None;
  let products = postpaid_v1::pricelist(Some(product_type_path), product_code).await.unwrap();
  println!("price list {:?}", products );

  // print products in json
  println!("price list JSON {}", serde_json::to_string(&products).unwrap() );
  


  Ok(())
}