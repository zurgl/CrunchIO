use crunchio::{CrunchIO, Result};

#[test]
#[ignore = "not ready yet"]
fn test_all_http_method_for_volumes() -> Result<()> {
  let client = CrunchIO::default();

  let volume_types = client.get_volumes_types()?;
  assert_ne!(volume_types.len(), 0);
  Ok(())
}
