use crate::Args;
use log::{debug, info};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn connect_mode(config: Args) -> anyhow::Result<()> {
    let addr_list: Vec<String> = config
        .port
        .iter()
        .map(|port| format!("{}:{}", config.host, port))
        .collect();

    for addr in addr_list {
        debug!("Connecting to {}", addr);

        let _stream = TcpStream::connect_timeout(
            &addr.to_socket_addrs()?.next().unwrap(),
            Duration::from_secs(config.timeout),
        )?;

        info!("Connected to {}", addr);
    }

    Ok(())
}
