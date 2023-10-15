#![allow(unused)]
#![allow(clippy::option_env_unwrap)]
#![allow(clippy::result_large_err)]
use crunchio::{blocking, schema::Currency, Result};

#[test]
fn test_all_http_method_for_user_balance() -> Result<()> {
  let id = option_env!("CLIENT_ID").unwrap();
  let secret = option_env!("CLIENT_SECRET").unwrap();

  let client = blocking::Client::new(id, secret)?;

  client.get_user_balance().map(|balance| {
    assert_eq!(balance.currency, Currency::USD);
    assert_ne!(balance.amount, -1f64);
  })
}
