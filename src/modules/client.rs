use super::types::{BikunaError, BikunaResult};

use tokio::net::TcpStream;

use std::net::SocketAddr;

pub async fn connect(address: SocketAddr) -> BikunaResult<TcpStream> {
    match TcpStream::connect(address).await {
        Ok(stream) => BikunaResult::Ok(stream),
        Err(err) => Err(BikunaError::Connection(err.to_string())),
    }
}
