use std::path::PathBuf;

#[derive(Debug)]
pub struct ParsedMatches {
	pub url: String,
	pub method: String,
	pub payload: String,
	pub output: Option<PathBuf>,
}

pub fn parse<'a> (incoming_matches: clap::ArgMatches<'a>) -> ParsedMatches {
  let url = incoming_matches.value_of("INPUT")
    .expect("INVALID URL / NO URL PROVIDED!");

  let method = incoming_matches.value_of("method")
    .unwrap_or("get");

	let output = if let Some(output) = incoming_matches.value_of("output") {
		let mut path = PathBuf::new();

		path.push(output);

		Some(path)
	} else {
		None
	};

	let payload = incoming_matches.value_of("payload")
		.unwrap_or_default();

  ParsedMatches {
		output,
		url: String::from(url),
		method: String::from(method),
		payload: String::from(payload),
	}
}
