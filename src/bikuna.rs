#[cfg(feature = "logger")]
#[macro_use]
extern crate log;
#[cfg(feature = "logger")]
extern crate chrono;
extern crate clap;
#[cfg(feature = "logger")]
extern crate colored;
extern crate tokio;

mod modules;
mod optionals;
#[macro_use]
mod utils;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use utils::{construct_address, parse_port};
use utils::to_log::*;

use std::net::SocketAddr;
use std::io::{self, BufRead};

#[tokio::main]
async fn main() -> modules::BikunaResult<()> {
    // Load in logger if the feature is enabled
    #[cfg(feature = "logger")]
    optionals::logger::enable().expect("Cannot set logger");
    let application = modules::app::generate();
    let matches = application.get_matches();

    let host = matches.value_of("address").unwrap();
    let port = matches.value_of("port").unwrap();

    let port = parse_port(port);

    let address = construct_address(host, port);

    let mut stream = forge_conn(address).await?;

    if matches.is_present("attach") {
        log_info("entering session...");

        attach(&mut stream).await
    } else if let Some(file) = matches.value_of("file") {
        let file_buff = modules::file_reader::read(file)?;

        write_message(&mut stream, file_buff.as_bytes(), matches.is_present("force_read")).await
    } else if let Some(msg) = matches.value_of("message") {
        write_message(&mut stream, msg.as_bytes(), matches.is_present("force_read")).await
    } else {
        Err(modules::types::BikunaError::Input(
            "Invalid Input...".to_owned(),
        ))
    }
}

async fn forge_conn(address: SocketAddr) -> modules::BikunaResult<TcpStream> {
    match modules::client::connect(address).await {
        Ok(stream) => Ok(stream),
        Err(err) => Err(modules::types::BikunaError::Connection(err.to_string())),
    }
}

async fn write_message(stream: &mut TcpStream, writer_buff: &[u8], force_read: bool) -> modules::BikunaResult<()> {
    if let Ok(bytes) = stream.write(writer_buff).await {

        log_info(&format!("wrote message to server and {} bytes!", bytes));

        if force_read {
            let mut reader: &mut [u8] = &mut [];

            if stream.read(&mut reader).await.is_ok() {
                log_info(&format!("read data from server: \n{:?}", reader));

                Ok(())
            } else {
                Err(
                    modules::types::BikunaError::Reader(
                        "Failed to read from server"
                            .to_owned()
                    )
                )
            }
        } else {
            Ok(())
        }
    } else {
        let err = modules::types::BikunaError::Writer("Fail to write message".to_owned());

        Err(err)
    }
}

pub async fn attach(stream: &mut TcpStream) -> modules::BikunaResult<()> {
    for line in io::stdin().lock().lines() {
        if write_message(stream, line.unwrap().as_bytes(), false).await.is_err() {
            log_info("cannot write over cmd");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::net::{IpAddr, Ipv4Addr};

    use super::construct_address as caddress;
    use super::modules::file_reader as fr;
    use super::parse_port as pport;

    #[test]
    fn parse_port() {
        assert_eq!(pport("3000"), 3000)
    }

    #[test]
    #[should_panic]
    fn parse_invalid_port() {
        pport("that's crazy nigga...");
    }

    #[test]
    fn construct_address() {
        assert_eq!(
            caddress("127.0.0.1", 3000),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000)
        )
    }

    #[test]
    #[should_panic]
    fn construct_invalid_address() {
        caddress("localhost", 3000);
    }

    #[test]
    fn read_from_file() {
        assert_eq!(fr::read("test-data/test.txt").is_ok(), true)
    }

    #[test]
    #[should_panic]
    fn read_from_non_existent_file() {
        fr::read("not/a/file.dumbass").unwrap();
    }
}
