use std::fmt::Error;

use iak_api::prepaid_v2;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let result = prepaid_v2::check_operator_prefix("081412345678".to_string()).unwrap();
  println!("operator {:?}", result );

  // print in json
  let json = serde_json::to_string(&result).unwrap();
  println!("operator json {}", json );

  Ok(())
}