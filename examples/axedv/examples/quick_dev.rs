#![allow(unused)]

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let hc = httpc_test::new_client("http://localhost:8080")?;

  hc.do_get("/hello").await?.print().await?;

  Ok(())
}
