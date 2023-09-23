use std::fmt::Error;

use iak_api::prepaid_v2;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let product_type_path = "data/telkomsel".to_string();
  let products = prepaid_v2::pricelist(Some(product_type_path)).await.unwrap();
  println!("price list {:?}", products );

  // print products in json
  println!("price list JSON {}", serde_json::to_string(&products).unwrap() );
  Ok(())
}