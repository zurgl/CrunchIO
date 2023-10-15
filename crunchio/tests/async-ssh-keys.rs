#![allow(unused)]
#![allow(clippy::option_env_unwrap)]
#![allow(clippy::result_large_err)]
use crunchio::{
  no_blocking,
  schema::{DeletedKeys, SshAddKeyBody, SshKey},
  Result,
};

#[tokio::test]
async fn test_all_http_method_for_ssh_keys() -> Result<()> {
  // let key = include_str!("assets/key.in");
  // println!("this is the : {key}");

  let id = option_env!("CLIENT_ID").unwrap();
  let secret = option_env!("CLIENT_SECRET").unwrap();

  let client = no_blocking::Client::new(id, secret).await?;

  let ssh_keys: Vec<SshKey> = client.get_all_ssh_keys().await?;
  println!("{ssh_keys:#?}");
  // assert_eq!(ssh_keys.len(), 1);
  /*

  let ssh_key = SshAddKeyBody::new()
    .set_name("tuoni")
    .set_key(include_str!("assets/key.in"))
    .to_owned();

  // let id = ssh_keys.first().unwrap().id;
  if let Some(SshKey { id, .. }) = ssh_keys.first() {
    if let Ok(key) = client.get_ssh_key_by_id(id).await {
      assert_eq!(key.len(), 1)
    }
  }

  let ssh_key_id = client.add_ssh_keys(&ssh_key).await?;
  assert!(!ssh_key_id.is_nil());

    let deleted: DeletedKeys = client.delete_ssh_key_by_id(&ssh_key_id).await?;
    assert_eq!(deleted.count, 1);

    let ssh_keys = client.get_all_ssh_keys().await?;
    assert_eq!(ssh_keys.len(), 1);

    let ssh_key_id = client.add_ssh_keys(name, key).await?;
    assert!(!ssh_key_id.is_nil());

    let ids = vec![ssh_key_id];
    let deleted = client.delete_ssh_keys(&ids).await?;
    assert_eq!(deleted.count, 1);
  */
  Ok(())
}
