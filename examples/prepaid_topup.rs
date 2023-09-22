use std::{fmt::Error, str::FromStr};
use iak_api::prepaid_v2;
use uuid::Uuid;



#[tokio::main]
async fn main() -> Result<(), Error> {
  // let ref_id: Uuid = uuid::Uuid::new_v4();
  let ref_id = Uuid::from_str("0327d0a7-782d-5cd4-926d-08fdebbef538").unwrap();
  let result = prepaid_v2::topup("hpln20000".to_string(), ref_id, "081850000000".to_string()).await.unwrap();
  println!("topup {:?}", result );
  // json

  let json = serde_json::to_string(&result).unwrap();
  println!("topup json {}", json );

  Ok(())
}