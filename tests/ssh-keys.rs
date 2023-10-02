use crunchio::CrunchIO;

#[test]
fn test_all_http_method_for_ssh_keys() {
  let client = CrunchIO::default();

  let ssh_keys = client.get_all_ssh_keys();
  assert_eq!(ssh_keys.len(), 1);

  let name = "new_startup";
  let key = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAAAgQC8orN63XfjrkUQUp7cye96/Lx/4gOfs6MzsKLCxX8ACDnXnI9eZkxXuJPr9i6Q+iXNkqcUTB9NyJnfuRx7okPFWvrpzRj3mEs9RZJQ+P3YSfblvxDe2AtRBTZtg2MP0j7afolthS6gdTfhSU+1oRSyvZz0zn0F7TCgem6vNekGLw== tuoni";

  let ssh_key_id = client.add_ssh_keys(name, key);
  assert!(!ssh_key_id.is_nil());

  let ssh_key = client.get_ssh_key_by_id(&ssh_key_id);
  assert_eq!(ssh_key.len(), 1);

  let deleted = client.delete_ssh_key_by_id(&ssh_key_id);
  assert_eq!(deleted.count, 1);

  let ssh_keys = client.get_all_ssh_keys();
  assert_eq!(ssh_keys.len(), 1);

  let ssh_key_id = client.add_ssh_keys(name, key);
  assert!(!ssh_key_id.is_nil());

  let ids = vec![ssh_key_id];
  let deleted = client.delete_ssh_keys(&ids);
  assert_eq!(deleted.count, 1);
}
