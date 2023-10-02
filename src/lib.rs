#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use openssl::ssl::{SslConnector, SslMethod};

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  let mut _ctx = SslConnector::builder(SslMethod::tls()).unwrap();
  
  a + b
}
