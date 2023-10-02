use crunchio::CrunchIO;

#[test]
fn test_all_http_method_for_instances() {
  let client = CrunchIO::default();

  let instances = client.get_all_instance_types();
  assert_ne!(instances.len(), 0);
}
