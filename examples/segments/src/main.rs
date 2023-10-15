#![allow(dead_code, unused)]

pub mod app {
  pub(crate) mod constants {
    pub(crate) const URL: &str = "https://api.datacrunch.io";
    pub(crate) const VERSION: &str = "v1";
    pub(crate) const IMAGES: [&str; 1] = ["images"];
    pub(crate) const BALANCE: [&str; 1] = ["balance"];
    pub(crate) const LOCATIONS: [&str; 1] = ["locations"];
    pub(crate) const INSTANCES: [&str; 1] = ["instances"];
    pub(crate) const INSTANCE_TYPES: [&str; 1] = ["instance-types"];
    pub(crate) const INSTANCE_AVAILABILITY: [&str; 1] =
      ["instance-availability"];
    pub(crate) const AUTHENTICATION: [&str; 2] = ["oauth2", "token"];
    pub(crate) const SSH_KEYS: [&str; 1] = ["sshkeys"];
    pub(crate) const SCRIPTS: [&str; 1] = ["scripts"];
    pub(crate) const VOLUMES: [&str; 1] = ["volumes"];
    pub(crate) const VOLUME_TYPES: [&str; 1] = ["volume-types"];
  }

  pub(crate) mod error {
    #[derive(thiserror::Error, Debug)]
    pub(crate) enum Error {
      #[error("Failed to parse the url")]
      Parsing(#[source] url::ParseError),

      #[error("Cannot add the segment")]
      AddSegment,

      #[error("Lib reqwest error")]
      Extend(#[source] url::ParseError),
    }
  }

  pub(crate) type Result<T, E = error::Error> = std::result::Result<T, E>;

  #[derive(Debug, Clone)]
  pub(crate) struct Route {
    url: url::Url,
  }

  impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.url)
    }
  }

  impl Route {
    pub(crate) fn try_default() -> Result<Self> {
      let url =
        url::Url::parse(constants::URL).map_err(error::Error::Parsing)?;
      Ok(Self { url })
    }

    pub(crate) fn set_version(&mut self) -> &mut Self {
      let url = &mut self.url;
      url.set_path(constants::VERSION);
      self
    }

    pub(crate) fn add_segment(&mut self, segment: &str) -> Result<&mut Self> {
      let url = &mut self.url;
      url
        .path_segments_mut()
        .map_err(|_| error::Error::AddSegment)?
        .push(segment);
      Ok(self)
    }

    pub(crate) fn add_segments<I>(&mut self, segments: I) -> Result<&mut Self>
    where
      I: IntoIterator,
      I::Item: AsRef<str>,
    {
      let url = &mut self.url;
      url
        .path_segments_mut()
        .map_err(|_| error::Error::AddSegment)?
        .extend(segments);
      Ok(self)
    }
  }
}

fn main() -> app::Result<()> {
  let mut route = app::Route::try_default()?;
  println!("{route}");

  route.set_version();
  println!("{route}");

  route.add_segment("coucou");
  println!("{route}");

  route.add_segments(["yacine", "dev"]);
  println!("{route}");

  let mut binding = app::Route::try_default()?;
  let new_route = binding
    .set_version()
    .add_segment("merde")?
    .add_segments(["turc", "much"])?;
  println!("{new_route}");

  Ok(())
}
