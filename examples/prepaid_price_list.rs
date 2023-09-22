use std::fmt::Error;

use iak_api::{prepaid_v2, ProductType};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let product_type_path = "data/telkomsel".to_string();
  let products = prepaid_v2::pricelist(Some(product_type_path)).await.unwrap();
  println!("price list {:?}", products );
  Ok(())
}