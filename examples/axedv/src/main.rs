#![allow(unused)]

use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
  let routes_hello = Router::new().route(
    "/hello",
    get(|| async { Html("hello <strong>World!!!</strong>") }),
  );

  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  println!("->> LISTENING on {addr}\n");
  axum::Server::bind(&addr)
    .serve(routes_hello.into_make_service())
    .await
    .unwrap();
}
