use std::fmt::Error;

use iak_api::prepaid_v2;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let result = prepaid_v2::inquiry_ovo("082179374708".to_string()).await.unwrap();
  println!("inquiry_ovo {:?}", result );
  // json

  let json = serde_json::to_string(&result).unwrap();
  println!("inquiry_ovo json {}", json );

  Ok(())
}