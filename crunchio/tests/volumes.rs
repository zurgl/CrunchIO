use crunchio::CrunchIO;

#[test]
fn test_all_http_method_for_volumes() {
  let client = CrunchIO::default();

  let volumes = client.get_all_volumes();
  assert_ne!(volumes.len(), 0);

  let volume_id = client.create_new_volume("test volume2", 50);
  println!("{volume_id:?}");

  let data = client.get_volume_by_id(&volume_id);
  println!("{data:?}");

  let info = client.delete_volume_by_id(&volume_id);
  println!("{info:?}");
}
