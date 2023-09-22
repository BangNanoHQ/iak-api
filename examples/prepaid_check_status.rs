use std::{fmt::Error, str::FromStr};
use iak_api::prepaid_v2;
use uuid::Uuid;



#[tokio::main]
async fn main() -> Result<(), Error> {
  // let ref_id: Uuid = uuid::Uuid::new_v4();
  let ref_id = Uuid::from_str("1d55426d-fad0-5808-974a-8d7fe73286ab").unwrap();
  let result = prepaid_v2::check_status(ref_id).await.unwrap();
  println!("check_status {:?}", result );
  // json

  let json = serde_json::to_string(&result).unwrap();
  println!("check_status json {}", json );

  Ok(())
}