use std::str::FromStr;
use std::default::Default;
#[cfg(feature = "https")]
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;

pub type MiniResponse = http::Result<http::Response<hyper::Body>>;
#[cfg(feature = "https")]
pub type MiniClient = hyper::Client<HttpsConnector<HttpConnector>>;
#[cfg(not(feature = "https"))]
pub type MiniClient = hyper::Client<HttpConnector>;

#[derive(Debug)]
pub struct RequestState<'a> {
	pub method: &'a [u8],
	pub client: MiniClient,
	pub payload: Option<String>,
	pub header_map: http::header::HeaderMap,
}

impl<'a> Default for RequestState<'a> {
	fn default () -> Self {
		Self {
			method: b"",
			payload: None,
			client: build_client(),
			header_map: http::header::HeaderMap::new(),
		}
	}
}

pub async fn request (
	url: &str,
	state: RequestState<'_>,
) -> MiniResponse {
	if let Ok(request_url) = hyper::Uri::from_str(url) {

		let method = match std::str::from_utf8(state.method) {
			Ok(meth) => meth,
			Err(_) => "",
		};

		let request = hyper::Request::builder()
			.uri(request_url)
			.method(method_from_str(method))
			.body(if let Some(body) = state.payload {
				hyper::Body::from(body)
			} else {
				hyper::Body::empty()
			})
			.expect("failed to build REQUEST - check the request then try again.");

		let response = state.client.request(request).await;

		Ok(response.expect("failed to send REQUEST"))
	} else {
		panic!("INVALID URL/URI - try entering the full address.")
	}
}

fn method_from_str (input: &str) -> http::Method {
	match input {
		"put" => http::Method::PUT,
		"post" => http::Method::POST,
		"head" => http::Method::TRACE,
		"patch" => http::Method::PATCH,
		"get" | "" => http::Method::GET,
		"delete" => http::Method::DELETE,
		"options" => http::Method::OPTIONS,
		&_ => http::Method::TRACE,
	}
}

#[cfg(feature = "https")]
fn build_client() -> MiniClient {
	let https = HttpsConnector::new();

	hyper::Client::builder().build::<_, hyper::Body>(https)
}

#[cfg(not(feature = "https"))]
fn build_client() -> MiniClient {
	hyper::Client::new()
}
