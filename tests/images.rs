use crunchio::CrunchIO;

#[test]
fn test_all_http_method_for_images() {
  let client = CrunchIO::default();

  let images = client.get_all_images_types();
  assert_ne!(images.len(), 0);
}
