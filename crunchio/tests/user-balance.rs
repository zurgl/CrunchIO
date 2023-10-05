use crunchio::{balance::Currency, CrunchIO, Result};

#[test]
fn test_all_http_method_for_user_balance() -> Result<()> {
  let client = CrunchIO::default();

  client.get_user_balance().map(|balance| {
    assert_eq!(balance.currency, Currency::USD);
    assert_ne!(balance.amount, -1f64);
  })
}
