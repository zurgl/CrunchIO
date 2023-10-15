#[derive(serde::Serialize, Debug, Default, Clone, PartialEq)]
pub struct SshAddKeyBody {
  pub(crate) name: String,
  pub(crate) key: String,
}

impl<'a> SshAddKeyBody {
  pub fn new() -> Self {
    SshAddKeyBody::default()
  }

  pub fn set_name(&'a mut self, name: &'a str) -> &mut SshAddKeyBody {
    self.name = name.to_owned();
    self
  }

  pub fn set_key(&'a mut self, key: &'a str) -> &mut SshAddKeyBody {
    self.key = key.to_owned();
    self
  }
}
