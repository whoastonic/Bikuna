use clap::{Arg, App, ArgMatches};

pub fn get_matches<'a> () -> ArgMatches<'a> {
  App::new("kuna")
    .version("v0.1.0-beta")
    .author("whoastonic <kingstonicthe1@gmail.com>")
    .about("Another terminal HTTP client, built in Rust")
    .args(&[
      Arg::with_name("method")
        .short("m")
        .long("config")
        .value_name("METHOD")
        .takes_value(true)
        .help("set the method for the request"),
      Arg::with_name("payload")
        .short("p")
        .long("payload")
        .value_name("PAYLOAD")
        .help("set the payload for the request")
        .takes_value(true),
      Arg::with_name("headers")
        .short("H")
        .long("headers")
        .value_name("HEADERS")
        .help("HEADER_KEY:HEADER_VALUE,HEADER_2_KEY:HEADER_2_VALUE")
        .takes_value(true),
      Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("OUTPUT")
        .help("out the reuqest body towards a file")
        .takes_value(true),
      Arg::with_name("INPUT")
        .help("set the URL for the request")
        .index(1)
        .required(true)
		])
		.get_matches()
}
