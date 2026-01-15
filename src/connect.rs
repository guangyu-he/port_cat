use log::{debug, info};
use serde::{Deserialize, Serialize};
use socket2::Socket;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::{Args, detect_service};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python", pyclass(dict, get_all, subclass))]
#[derive(Serialize, Deserialize, Debug, Clone)]
/// Information about a connection
/// Attributes:
/// * `host` - The host connected to
/// * `port` - The port connected to
/// * `timeout` - Connection timeout in seconds
/// * `recv_buffer_size` - Size of the receive buffer
/// * `send_buffer_size` - Size of the send buffer
/// * `keepalive` - Whether keepalive is enabled
/// * `reuse_address` - Whether address reuse is enabled
/// * `remote_ip` - Remote IP address
/// * `remote_port` - Remote port number
/// * `service` - Detected service on the port
pub struct ConnectionInfo {
    host: String,
    port: u16,
    timeout: u64,
    recv_buffer_size: usize,
    send_buffer_size: usize,
    keepalive: bool,
    reuse_address: bool,
    remote_ip: String,
    remote_port: u16,
    service: Option<String>,
}

#[cfg_attr(feature = "python", pymethods)]
impl ConnectionInfo {
    pub fn __repr__(&self) -> String {
        format!(
            "ConnectionInfo(host='{}', port={}, remote_ip='{}', service={})",
            self.host,
            self.port,
            self.remote_ip,
            match &self.service {
                Some(s) => format!("'{}'", s),
                None => "None".to_string(),
            }
        )
    }
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (ports, host, timeout=5))]
/// Connect mode function, Python binding
/// # Arguments
/// * `ports` - A vector of port numbers to connect to
/// * `host` - The host to connect to
/// * `timeout` - Connection timeout in seconds
/// Returns:
/// * `Ok(())` on success, or an error on failure
pub fn connect_mode(
    ports: Vec<u16>,
    host: String,
    timeout: Option<u64>,
) -> PyResult<Vec<ConnectionInfo>> {
    _connect_mode(ports, host, timeout.unwrap_or(5))
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))
}

/// Connect mode function, CLI binding
/// # Arguments
/// * `config` - The command line arguments
/// Returns:
/// * `Ok(())` on success, or an error on failure
pub fn connect_mode_cli(config: Args) -> anyhow::Result<()> {
    let _ = _connect_mode(config.port, config.host, config.timeout)?;
    Ok(())
}

/// Internal connect mode function
/// # Arguments
/// * `ports` - A vector of port numbers to connect to
/// * `host` - The host to connect to
/// * `timeout` - Connection timeout in seconds
/// Returns:
/// * `Ok(Vec<ConnectionInfo>)` on success, or an error on failure
fn _connect_mode<H: AsRef<str>>(
    ports: Vec<u16>,
    host: H,
    timeout: u64,
) -> anyhow::Result<Vec<ConnectionInfo>> {
    let addr_list: Vec<String> = ports
        .iter()
        .map(|port| format!("{}:{}", host.as_ref(), port))
        .collect();

    let mut results = Vec::with_capacity(addr_list.len());
    for (i, addr) in addr_list.iter().enumerate() {
        debug!("Connecting to {}", addr);
        let mut stream = TcpStream::connect_timeout(
            &addr.to_socket_addrs()?.next().unwrap(),
            Duration::from_secs(timeout),
        )?;
        info!("Connected to {}", addr);

        debug!("Detecting service on port {}", ports[i]);
        let port = ports[i];
        let service = detect_service::detect_service(&mut stream, port);
        info!("{} Service detected: {}", addr, service);

        let socket = Socket::from(stream.try_clone()?);
        let peer = stream.peer_addr()?;

        results.push(ConnectionInfo {
            host: host.as_ref().to_string(),
            port: ports[i],
            timeout,
            recv_buffer_size: socket.recv_buffer_size()?,
            send_buffer_size: socket.send_buffer_size()?,
            keepalive: socket.keepalive()?,
            reuse_address: socket.reuse_address()?,
            remote_ip: peer.ip().to_string(),
            remote_port: peer.port(),
            service: Some(service),
        });
    }

    Ok(results)
}
