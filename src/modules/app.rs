use clap::{App, Arg};

pub fn generate<'a, 'b>() -> App<'a, 'b> {
    App::new("Bikuna")
        .version("0.1.0")
        .author("whoastonic <kingstonicthe1@gmail.com>")
        .about("Simple TCP client CLI for basic messaging...")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .default_value("127.0.0.1")
                .help("Supply a host to connection opts")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Supply port to connection opts")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("MESSAGE")
                .required_unless_one(&["file", "attach"])
                .help("Write via message to stream")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("force_read")
                .short("r")
                .long("force_read")
                .value_name("FORCE_READ")
                .takes_value(false)
                .help("Will wait until server sends back a response...")
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .required_unless_one(&["file", "attach"])
                .help("Writes a file to stream")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("attach")
                .short("a")
                .long("attach")
                .value_name("ATTACH")
                .help("Attaches to TCP client sesssion, until connection closes")
                .takes_value(false),
        )
}
