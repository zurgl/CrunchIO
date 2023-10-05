use crunchio::{locations::LocationCode, CrunchIO};

#[test]
fn test_all_http_method_for_instance_availability() {
  let client = CrunchIO::default();

  assert_ne!(
    client
      .get_instance_availabilities(true, &LocationCode::FIN01)
      .len(),
    0
  );
  assert_ne!(
    client
      .get_instance_availabilities(true, &LocationCode::ICE01)
      .len(),
    0
  );
  assert_ne!(
    client
      .get_instance_availabilities(false, &LocationCode::FIN01)
      .len(),
    0
  );
  assert_ne!(
    client
      .get_instance_availabilities(false, &LocationCode::ICE01)
      .len(),
    0
  );

  let binding = client.get_instance_availabilities(false, &LocationCode::ICE01);
  let available_instance_type = binding.first().unwrap().availabilities.get(0).unwrap();

  let is_available = client.get_instance_availability_by_type(available_instance_type);
  assert!(is_available)
}
