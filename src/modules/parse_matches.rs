use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParsedMatches {
	pub url: String,
	pub method: String,
  // pub headers: HashMap<String, String>
}

pub fn parse<'a> (incoming_matches: clap::ArgMatches<'a>) -> ParsedMatches {
  let url = incoming_matches.value_of("INPUT")
    .expect("INVALID URL / NO URL PROVIDED!");

  let method = incoming_matches.value_of("method")
    .unwrap_or("get");

  ParsedMatches {
		url: String::from(url),
		method: String::from(method),
	}
}
