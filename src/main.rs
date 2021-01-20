#[macro_use]
extern crate clap;
extern crate tokio;
extern crate lib_kuna;

mod modules;

use std::default::Default;

use modules::parse_matches;
use modules::clap_config as app;
use lib_kuna::khttp as mini_http;
use hyper::body::HttpBody as _;

#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {
	let matches = app::get_matches();

	let parsed_matches = parse_matches::parse(matches);

	let mut response = perform_request(&parsed_matches.method, &parsed_matches.url).await
		.expect("cannot peform request - invalid cause");

	while let Some(chunk) = response.body_mut().data().await {
		dbg!(&chunk);
	}

	Ok(())
}

async fn perform_request (method: &str, url: &str) -> mini_http::MiniResponse {
	mini_http::request(url, mini_http::RequestState {
		method: method.as_bytes(),
		..Default::default()
	}).await
}
