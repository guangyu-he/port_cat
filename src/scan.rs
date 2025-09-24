use crate::Args;
use log::{debug, info, warn};
use std::io;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn scan_mode(config: Args) -> anyhow::Result<()> {
    let port_range = if let Some(range_str) = config.range {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid range format. Use start-end."));
        }
        let start: u16 = parts[0].parse()?;
        let end: u16 = parts[1].parse()?;
        if start > end {
            return Err(anyhow::anyhow!("Invalid range: start must be <= end."));
        }
        Some((start, end))
    } else {
        None
    };

    let (start_port, end_port) = if let Some((start, end)) = port_range {
        (start, end)
    } else {
        return Err(anyhow::anyhow!(
            "Please provide a valid port range with --range"
        ));
    };

    info!("Scanning {}:{}-{}", config.host, start_port, end_port);

    let mut open_ports = Vec::new();

    for port in start_port..=end_port {
        let addr = format!("{}:{}", config.host, port);
        debug!("Scanning port {}... ", port);
        io::stdout().flush().unwrap();

        match TcpStream::connect_timeout(
            &addr.to_socket_addrs()?.next().unwrap(),
            Duration::from_millis(100),
        ) {
            Ok(_) => {
                open_ports.push(port);
                info!("Port {} OPEN", port);
            }
            Err(_) => {
                warn!("Port {} CLOSED", port);
            }
        }
    }

    if open_ports.is_empty() {
        info!("No open ports found in range {}-{}", start_port, end_port);
    } else {
        info!("Found {} open ports: {:?}", open_ports.len(), open_ports);
    }

    Ok(())
}
