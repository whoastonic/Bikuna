#[macro_use]
pub mod to_log;

use std::net::SocketAddr;
 
pub fn construct_address(host: &str, port: i32) -> SocketAddr {
    let address = format!("{}:{}", host, port);

    let sock_addr: SocketAddr = address.parse().expect("Invalid Address");

    sock_addr
}

pub fn parse_port(port: &str) -> i32 {
    port.parse::<i32>().expect("Invalid port")
}
