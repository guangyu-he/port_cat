use crate::{Args, detect_service};
use log::{debug, info};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn connect_mode(config: Args) -> anyhow::Result<()> {
    let addr_list: Vec<String> = config
        .port
        .iter()
        .map(|port| format!("{}:{}", config.host, port))
        .collect();

    for (i, addr) in addr_list.iter().enumerate() {
        debug!("Connecting to {}", addr);

        let mut stream = TcpStream::connect_timeout(
            &addr.to_socket_addrs()?.next().unwrap(),
            Duration::from_secs(config.timeout),
        )?;
        info!("Connected to {}", addr);

        debug!("Detecting service on port {}", config.port[i]);
        let port = config.port[i];
        let service = detect_service::detect_service(&mut stream, port);

        info!("{} Service detected: {}", addr, service);
    }

    Ok(())
}
