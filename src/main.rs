#[macro_use]
extern crate log;
extern crate clap;
extern crate tokio;
extern crate lib_kuna;

mod modules;

use std::default::Default;

pub use lib_kuna::http_client;
use http_client::RequestState;
use modules::parse_matches;
use modules::clap_config as app;
use hyper::body::HttpBody as _;
use lib_kuna::logger::build as logger_build;

use log::LevelFilter::Info as LoggingFilter;

#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {
	logger_build(LoggingFilter.to_level().unwrap()).expect("cannot build logger");

	let matches = app::get_matches();

	let parsed_matches = parse_matches::parse(matches);

	let mut response = perform_request(&parsed_matches.method, &parsed_matches.url).await
		.expect("cannot peform request - invalid cause");

	while let Some(chunk) = response.body_mut().data().await {
		info!("{:?}", &chunk.unwrap());
	}

	Ok(())
}

async fn perform_request (method: &str, url: &str) -> http_client::MiniResponse {
	http_client::request(url, RequestState {
		method: method.as_bytes(),
		..Default::default()
	}).await
}
