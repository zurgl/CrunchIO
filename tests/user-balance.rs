use crunchio::{balance::Currency, CrunchIO};

#[test]
fn test_all_http_method_for_user_balance() {
  let client = CrunchIO::default();

  let balance = client.get_user_balance();
  assert_eq!(balance.currency, Currency::USD);
  assert_ne!(balance.amount, -1f64);
}
