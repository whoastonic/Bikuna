#[macro_use]
extern crate serde;
extern crate tokio;
#[macro_use]
extern crate clap;

use std::env;
use std::thread;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, Shutdown};

use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use clap::{App, Arg, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Bikuna")
        .version("1.0")
        .author("whoastonic <kingstonicthe1@gmail.com>")
        .about("A simple & lazy kid, doing simple & lazy things")
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("ADDRESS")
            .help("Server's address")
            .takes_value(true))
        .arg(Arg::with_name("execute")
            .short("e")
            .long("execute")
            .value_name("EXECUTE")
            .help("Oneshot command execute")
            .takes_value(true)
        ).get_matches();    
        
    let address = matches.value_of("address")
        .unwrap_or("127.0.0.1:65304");

    let exec =  matches.value_of("execute")
        .unwrap_or("");

    let addr = address.parse::<SocketAddr>()
        .expect("Invalid Address | Fail to parse");

    let mut message = String::new();

    if !exec.is_empty() {
        if let Ok(mut stream) = TcpStream::connect(&addr).await {
            stream.write_all(exec.as_bytes()).await?;
        } else {
            eprintln!("Seems like server/socket isn't open")
        }
    } else {
        if let Ok(mut stream) = TcpStream::connect(&addr).await {
            println!("Connected to server w/port: {:?}", stream.peer_addr()?);

            stream.read_to_string(&mut message).await?;

            let message: Vec<&str> = message.split(" ").collect();
            let cmd = message[0];
            let args = &message[1..];

            while cmd != "!exit" {
                match cmd {
                    "!write" => {
                        println!("Do stuff here...")
                    },
                    _ => {}
                }
            }

            stream.shutdown(Shutdown::Write)?;
        } else {
            eprintln!("Seems like server/socket isn't open")
        }
    }

    Ok(())
}

