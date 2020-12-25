#[macro_use]
extern crate log;

use std::thread::sleep;
use std::time::Duration;

use tokio::net::TcpStream;
use tokio::prelude::*;

use buruma::constants::Error;
use buruma::ZKResult;
use std::thread;

async fn mock_fn(addr: &str) -> ZKResult<String> {
    // let mut server_list = Vec::new();
    // server_list.push(addr.to_string());
    //
    // let socket = match TcpStream::connect(server_list.get(0).unwrap().as_str()).await {
    //     Ok(socket) => socket,
    //     Err(e) => return Ok(addr.to_string()),
    // };
    //
    // let (mut reader, mut writer) = io::split(socket);

    tokio::spawn(async move {
        loop {
            info!("writing!!!");
            thread::sleep(Duration::from_millis(500));
        }
        Ok::<_, io::Error>(())
    });
    info!("after spawn");
    Ok(addr.to_string())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("tokio");
    mock_fn("127.0.0.1:2181").await.unwrap();
    thread::sleep(Duration::from_secs(10));
    info!("after mock");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn new_test() {
        pretty_env_logger::init();
        info!("tokio");
        mock_fn("127.0.0.1:2181").await.unwrap();
        thread::sleep(Duration::from_secs(10));
        info!("after mock");
    }
}