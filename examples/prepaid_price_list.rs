use std::fmt::Error;

use iak_api::prepaid_v2;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let products = prepaid_v2::pricelist().await.unwrap();
  println!("price list {:?}", products );
  Ok(())
}