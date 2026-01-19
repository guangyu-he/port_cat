use crate::{Args, detect_service};
use anyhow::{Result, anyhow};
use futures::future::join_all;
use log::{debug, info};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_async_runtimes::tokio::future_into_py;

/// Scan a single port on the given host
/// # Arguments
/// * `port` - The port number to scan
/// * `host` - The host to scan
/// # Returns
/// * `Result<u16>` - The open port number if successful, error otherwise
async fn every_port_scan<S: AsRef<str>>(port: u16, host: S) -> Result<u16> {
    let addr = format!("{}:{}", host.as_ref(), port);
    debug!("Scanning port {}... ", port);

    let stream_res = TcpStream::connect_timeout(
        &addr.to_socket_addrs()?.next().unwrap(),
        Duration::from_millis(100),
    );

    match stream_res {
        Ok(mut stream) => {
            info!("Port {} OPEN", port);
            let service = detect_service::detect_service(&mut stream, port);
            info!("Port {} Service detected: {}", port, service);
            Ok(port)
        }
        Err(_) => {
            debug!("Port {} CLOSED", port);
            Err(anyhow!("Port {} CLOSED", port))
        }
    }
}

/// Scan mode entry point for CLI
/// # Arguments
/// * `config` - The command line arguments
/// # Returns:
/// * `Result<()>` - Ok on success, error otherwise
pub async fn scan_mode_cli(config: Args) -> Result<()> {
    let _ = _scan_mode(config.scan, config.host).await?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (host, scan_range="0-65535".to_string()))]
/// Scan mode function, Python binding
/// # Arguments
/// * `host` - The host to scan
/// * `scan_range` - The port range to scan in the format "start-end"
/// # Returns:
/// * `PyResult<Vec<u16>` (bound with async) - Ok on success, error otherwise
pub fn scan_mode<'p>(
    py: Python<'p>,
    host: String,
    scan_range: Option<String>,
) -> PyResult<Bound<'p, PyAny>> {
    future_into_py(py, async move {
        let ports = _scan_mode(scan_range, host)
            .await
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()));
        ports
    })
}

/// Internal scan mode function
/// # Arguments
/// * `scan_range` - The port range to scan in the format "start-end"
/// * `host` - The host to scan
/// # Returns:
/// * `Result<()>` - Ok on success, error otherwise
async fn _scan_mode<S: Into<String>>(scan_range: Option<S>, host: S) -> Result<Vec<u16>> {
    let port_range = if let Some(range_str) = scan_range {
        let range_str: &str = &range_str.into();
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

    let host: Arc<str> = Arc::from(host.into());
    info!("Scanning {}:{}-{}", host, start_port, end_port);

    let handles = (start_port..end_port)
        .map(|port| {
            let host = Arc::clone(&host);
            tokio::spawn(async move { every_port_scan(port, host).await })
        })
        .collect::<Vec<_>>();

    // wait for all scan tasks to complete and collect results
    let results = join_all(handles).await;

    // filter open ports from results and collect them
    let open_ports: Vec<u16> = results
        .into_iter()
        .filter_map(|handle_result| {
            match handle_result {
                Ok(scan_result) => scan_result.ok(), // if scan successful and open, return port
                Err(_) => None,                      // failed task, ignore
            }
        })
        .collect();

    if open_ports.is_empty() {
        info!("No open ports found in range {}-{}", start_port, end_port);
    } else {
        info!("Found {} open ports: {:?}", open_ports.len(), open_ports);
    }

    Ok(open_ports)
}
