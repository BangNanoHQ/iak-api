use std::{fmt::Error, str::FromStr};
use iak_api::prepaid_v2;
use uuid::Uuid;



#[tokio::main]
async fn main() -> Result<(), Error> {
  // let ref_id: Uuid = uuid::Uuid::new_v4();
  let ref_id = Uuid::from_str("c8265814-166e-4660-a84a-69f8a26be2d5").unwrap();
  let result = prepaid_v2::topup("pulsa100000".to_string(), ref_id, "081850000000".to_string()).await.unwrap();
  println!("topup {:?}", result );
  // json

  let json = serde_json::to_string(&result).unwrap();
  println!("topup json {}", json );

  Ok(())
}