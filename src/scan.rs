use crate::{Args, detect_service};
use anyhow::{Result, anyhow};
use futures::future::join_all;
use log::{debug, info, warn};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

async fn every_port_scan(port: u16, host: String) -> Result<u16> {
    let addr = format!("{}:{}", host, port);
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
            warn!("Port {} CLOSED", port);
            Err(anyhow!("Port {} CLOSED", port))
        }
    }
}

pub async fn scan_mode(config: Args) -> Result<()> {
    let port_range = if let Some(range_str) = config.scan {
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

    let handles = (start_port..end_port)
        .map(|port| {
            let host = config.host.clone();
            tokio::spawn(async move { every_port_scan(port, host).await })
        })
        .collect::<Vec<_>>();

    // 等待所有任务完成并收集结果
    let results = join_all(handles).await;

    // 过滤成功的结果，得到开放的端口列表
    let open_ports: Vec<u16> = results
        .into_iter()
        .filter_map(|handle_result| {
            match handle_result {
                Ok(scan_result) => scan_result.ok(), // 如果任务成功且端口开放
                Err(_) => None,                      // 任务失败
            }
        })
        .collect();

    if open_ports.is_empty() {
        info!("No open ports found in range {}-{}", start_port, end_port);
    } else {
        info!("Found {} open ports: {:?}", open_ports.len(), open_ports);
    }

    Ok(())
}
