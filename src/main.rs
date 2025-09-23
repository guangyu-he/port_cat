use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use log::{debug, info, warn};
use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Host to connect to
    #[clap(index = 1, default_value = "localhost")]
    host: String,

    /// Port number
    #[clap(short, long, value_delimiter = ',', default_value = "80,443")]
    port: Vec<u16>,

    /// Scan mode
    #[clap(short, long, default_value_t = false)]
    scan: bool,

    /// Port range for scanning (e.g., 1-1000)
    #[clap(short, long)]
    range: Option<String>,

    /// Timeout in seconds
    #[clap(short, long, default_value_t = 5)]
    timeout: u64,

    /// Verbose output
    #[clap(long, default_value = "info")]
    debug_level: String,
}

fn main() {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(args.debug_level.as_str()))
        .format_timestamp_secs()
        .init();

    match run(args) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(args: Args) -> Result<()> {
    if args.scan {
        return port_scan(args);
    }

    connect_mode(args)
}

fn connect_mode(config: Args) -> Result<()> {
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

fn port_scan(config: Args) -> Result<()> {
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
        info!("Scanning port {}... ", port);
        io::stdout().flush().unwrap();

        match TcpStream::connect_timeout(
            &addr.to_socket_addrs()?.next().unwrap(),
            Duration::from_millis(100),
        ) {
            Ok(_) => {
                open_ports.push(port);
                info!("OPEN");
            }
            Err(_) => {
                warn!("CLOSED");
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
