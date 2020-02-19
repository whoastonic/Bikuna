extern crate tokio;
extern crate clap;

use tokio::net::TcpStream;
use tokio::prelude::*;
use clap::{App, Arg};

use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = String::new();

    let matches = generate_app().get_matches();    

    let p = matches.value_of("port").unwrap_or("8080");
    port.push_str(p);

    let addr: SocketAddr = format!("127.0.0.1:{}", port)
        .parse()
        .unwrap();

    if let Ok(mut stream) = TcpStream::connect(&addr).await {
        if let Some(msg) = matches.value_of("message") {
            let mut m = String::from(msg);

            if m.starts_with(">") {
                let segments: Vec<&str> = msg.split(" ").collect();

                let file = segments[1];
            
                m = read_file(file);
            }

            println!("Writing: ({}) over to server", m);

            if let Err (err) = stream.write(m.as_bytes()).await {
                eprintln!("Error writing data to server: {:?}", err);
            } else {
                let mut b = [0; 250];

                stream.read(&mut b).await?;

                let result = std::str::from_utf8(&b).unwrap();

                println!("Got back result: {:?}", result);
            }
        }
    } else {
        println!("Unable to connect to this socket, are you sure it's open?")
    }

    Ok(())
}

fn read_file(file: &str) -> String {
    let mut contents = String::new();
    let mut file = File::open(file).unwrap();

    file.read_to_string(&mut contents)
        .expect("Cannot read file to string");

    dbg!(&contents);

    contents
}

fn generate_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Bikuna")
        .version("0.1.0")
        .author("whoastonic <kingstonicthe1@gmail.com>")
        .about("Doing stupid, lazy and pointless stuff")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Connect to socket server")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("msg")
                .value_name("MESSAGE")
                .help("Write via message to stream")
                .takes_value(true)  
        )
}
