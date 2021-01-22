extern crate hyper;
#[cfg(feature = "https")]
extern crate hyper_tls;
extern crate log;
extern crate http;
extern crate tokio;

pub mod http_client;
mod utils;
pub use utils::*;
