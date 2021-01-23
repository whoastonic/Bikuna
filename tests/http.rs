extern crate serde;
extern crate lib_kuna;
extern crate serde_json;

use serde::{Serialize, Deserialize};

use std::default::Default;
use lib_kuna::http_client;
use http_client::RequestState;

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
	hello: String,
	world: String,
}

impl RequestBody {
	pub fn new (construct: (String, String)) -> Self {
		Self { hello: construct.0, world: construct.1 }
	}
}

#[test]
fn get_request () {
  let future = http_client::request("https://example.com", RequestState {
		..Default::default()
	});

	let success = async_block!(async {
		if let Ok(response) = future.await {
			response.status().is_success()
		} else {
			false
		}
	});

	assert_eq!(success, true)
}

#[test]
fn post_request () {
	let request_body = body_to_string::<RequestBody>(
		RequestBody::new((String::from("anime"), String::from("girl")))
	);

	let future = http_client::request("https://jsonplaceholder.typicode.com/posts", RequestState {
		method: b"post",
		payload: Some(request_body),
		..Default::default()
	});

	let success = async_block!(async {
		if let Ok(response) = future.await {
			response.status().is_success()
		} else {
			false
		}
	});

	assert!(success)
}

#[test]
fn post_with_file_request () {


}

fn body_to_string<T: Serialize> (body: T) -> String {
	serde_json::to_string(&body).unwrap()
}

#[macro_export]
macro_rules! async_block {
	($e:expr) => {
		tokio_test::block_on($e)
	};
}
