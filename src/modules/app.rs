use clap::{App, Arg};

pub fn generate<'a, 'b>() -> App<'a, 'b> {
    App::new("Bikuna")
        .version("0.1.0")
        .author("whoastonic <kingstonicthe1@gmail.com>")
        .about("Simple TCP client CLI for basic messaging...")
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("ADDRESS")
                .default_value("127.0.0.1")
                .help("Suppy address to connection opts [ DEFAULTS TO 127.0.0.1 ]")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Supply port to connection opts")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("MESSAGE")
                .help("Write via message to stream")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Writes a file to stream")
                .takes_value(true)
        )
}
