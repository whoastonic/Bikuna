#[cfg(feature="logger")]
#[macro_use]
extern crate log;
#[cfg(feature="logger")]
extern crate colored;
#[cfg(feature="logger")]
extern crate chrono;
extern crate tokio;
extern crate clap;

mod utils;
mod modules;
mod optionals;

use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use utils::*;

use std::net::SocketAddr;
use std::time::Duration;
use std::thread;

#[tokio::main]
async fn main() -> modules::BikunaResult<()> {
    // Load in logger if the feature is enabled
    #[cfg(feature="logger")]
    optionals::logger::enable().expect("Cannot set logger");

    let application = modules::app::generate();
    let matches = application.get_matches();

    let host = matches.value_of("address").unwrap();
    let port = matches.value_of("port").unwrap();

    let port = parse_port(port);

    let address = construct_address(host, port);

    let mut stream = forge_conn(address).await?;

    if matches.is_present("attach") {
        thread::sleep(Duration::from_millis(5000));
        Ok(())
    } else if let Some(file) = matches.value_of("file") {
            let file_buff = modules::file_reader::read(file)?;

            write_message(&mut stream, file_buff.as_bytes()).await
    } else if let Some(msg) = matches.value_of("message") {
            write_message(&mut stream, msg.as_bytes()).await
    } else {
        Err(
            modules::types::BikunaError::Input(
                "Invalid Input...".to_owned()
            )
        )
    }
    
    // let address = construct_address(host, port);
    //     if let Some(matched_file) = matches.value_of("file") {
    //             match modules::file_reader::read(matched_file) {
    //                 Ok(buff) => write_message(&mut stream, buff.as_bytes()).await,
    //                 Err(err) => Err(
    //                     modules::types::BikunaError::Reader(err.to_string())
    //                 )
    //             }
    //         } else if let Some(matched_msg) = matches.value_of("message") {
    //             write_message(&mut stream, matched_msg.as_bytes()).await
    //         } else {
    //             Err(
    //                 modules::types::BikunaError::Input(
    //                     "No valid input provided...".to_owned()
    //                 )
    //             )
    //         }      
}

async fn forge_conn(address: SocketAddr) -> modules::BikunaResult<TcpStream> {
    match modules::client::connect(address).await {
        Ok(stream) => Ok(stream),
        Err(err) => Err(
            modules::types::BikunaError::Connection(
                err.to_string()
            )
        )
    }
}

async fn write_message(stream: &mut TcpStream, writer_buff: &[u8]) -> modules::BikunaResult<()> {
    if let Ok(bytes) = stream.write(writer_buff).await {
        let mut reader_buff: &mut [u8] = &mut []; 

        info!("wrote message to server and {} bytes!", bytes);
    
        if let Ok(_) = stream.read(&mut reader_buff).await {
            info!("read data from server:\n{:?}", reader_buff);

            Ok(())
        } else {
            Err(
                modules::types::BikunaError::Reader(
                    "Fail to read from server".to_owned()
                )
            )
        }
    } else {
        let err = modules::types::BikunaError::Writer("Fail to write message".to_owned());

        Err(err)
    }
}


#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::net::{IpAddr, Ipv4Addr};

    use super::parse_port as pport;
    use super::construct_address as caddress;
    use super::modules::file_reader as fr;

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
            SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                3000
            )
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
