use crunchio::CrunchIO;

#[test]
fn test_all_http_method_for_startup_script() {
  let client = CrunchIO::default();

  let name = "new_startup";
  let script = r"
  #!/bin/bash
  apt update
  apt upgrade -y
  ";

  let startup_script_id = client.add_startup_script(name, script);
  assert!(!startup_script_id.is_nil());

  assert_eq!(client.get_all_startup_scripts().len(), 1);

  assert_eq!(client.get_startup_script_by_id(&startup_script_id).len(), 1);

  assert_eq!(
    client.delete_startup_script_by_id(&startup_script_id).count,
    1
  );

  let startup_script_id = client.add_startup_script(name, script);
  assert!(!startup_script_id.is_nil());

  assert_eq!(
    client
      .delete_startup_scripts(&vec![startup_script_id])
      .count,
    1
  );
}
