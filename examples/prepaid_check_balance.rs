use std::fmt::Error;

use iak_api::prepaid_v2;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let result = prepaid_v2::check_balance().await.unwrap();
  println!("check_balance {:?}", result );
  Ok(())
}