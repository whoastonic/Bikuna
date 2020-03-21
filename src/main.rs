extern crate tokio;
extern crate clap;

mod modules;

use tokio::io::AsyncWriteExt;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> modules::BikunaResult<()> {
    let application = modules::app::generate();
    let matches = application.get_matches();

    let matched_host = matches.args.get("address").unwrap();
    let matched_port = matches.args.get("port").unwrap();

    let host = matched_host.vals[0].to_str().unwrap();
    let port = matched_port.vals[0].to_str().unwrap();  
    let port = parse_port(port);

    let address = construct_address(host, port);

    match modules::client::connect(address).await {
        Ok(mut stream) => {
            if let Some(matched_file) = matches.value_of("file") {
                match modules::file_reader::read(matched_file) {
                    Ok(buff) => if let Ok(bytes) = stream.write(buff.as_bytes()).await {
                        println!("wrote message to server and {} bytes!", bytes);
                        Ok(())
                    } else {
                        Err(
                            modules::types::BikunaError::Writer("Fail to write message".to_owned())
                        )
                    },
                    Err(err) => Err(
                        modules::types::BikunaError::Reader(err.to_string())
                    )
                }
            } else {
                if let Some(matched_msg) = matches.value_of("message") {
                    if let Ok(bytes) = stream.write(matched_msg.as_bytes()).await {
                        println!("wrote message to server and {} bytes!", bytes);
                        Ok(())
                    } else {
                        Err(
                            modules::types::BikunaError::Writer("Fail to write message".to_owned())
                        )
                    }
                } else {
                    Err(
                        modules::types::BikunaError::Input(
                            "No valid input provided...".to_owned()
                        )
                    )
                } 
            }
        },
        Err(err) => Err(modules::types::BikunaError::Connection(err.to_string()))
        
    }
 
}

fn construct_address(host: &str, port: i32) -> SocketAddr {
    let address = format!("{}:{}", host, port);

    let sock_addr: SocketAddr = address.parse()
        .expect("Invalid Address");
    
    sock_addr
}

fn parse_port(port: &str) -> i32 {
    port.parse::<i32>().expect("Invalid port")
}
