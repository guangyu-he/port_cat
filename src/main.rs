use anyhow::Result;
use std::env;
use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    scan: bool,
    port_range: Option<(u16, u16)>,
    timeout: u64,
    verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "localhost".to_string(),
            port: 80,
            scan: false,
            port_range: None,
            timeout: 5,
            verbose: false,
        }
    }
}

fn main() {
    let config = parse_args();

    match run(config) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::default();
    let mut i = 1;

    if args.len() == 1 {
        print_usage();
        std::process::exit(0);
    }

    while i < args.len() {
        match args[i].as_str() {
            "-p" | "--port" => {
                if i + 1 < args.len() {
                    config.port = args[i + 1].parse().unwrap_or(config.port);
                    i += 1;
                }
            }
            "-s" | "--scan" => {
                config.scan = true;
            }
            "-r" | "--range" => {
                if i + 1 < args.len() {
                    let range_str = &args[i + 1];
                    if let Some((start_str, end_str)) = range_str.split_once('-') {
                        let start: u16 = start_str.parse().unwrap_or(1);
                        let end: u16 = end_str.parse().unwrap_or(65535);
                        config.port_range = Some((start, end));
                    }
                    i += 1;
                }
            }
            "-t" | "--timeout" => {
                if i + 1 < args.len() {
                    config.timeout = args[i + 1].parse().unwrap_or(config.timeout);
                    i += 1;
                }
            }
            "-v" | "--verbose" => {
                config.verbose = true;
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                if !args[i].starts_with('-') && config.host == "localhost" {
                    config.host = args[i].clone();
                }
            }
        }
        i += 1;
    }

    config
}

fn print_usage() {
    println!("Rust Netcat - Network utility tool");
    println!();
    println!("Usage: rustcat [OPTIONS] [HOST]");
    println!();
    println!("Options:");
    println!("  -p, --port PORT        Specify port number");
    println!("  -s, --scan             Port scan mode");
    println!("  -r, --range START-END  Port range for scanning (e.g., 1-1000)");
    println!("  -t, --timeout SECONDS  Connection timeout (default: 10)");
    println!("  -v, --verbose          Verbose output");
    println!("  -h, --help             Show this help message");
    println!();
    println!("Examples:");
    println!("  rustcat example.com 80             # Connect to example.com:80");
    println!("  rustcat -s example.com -r 1-1000   # Scan ports 1-1000");
}

fn run(config: Config) -> Result<()> {
    if config.scan {
        return port_scan(&config);
    }

    connect_mode(&config)
}

fn connect_mode(config: &Config) -> Result<()> {
    let addr = format!("{}:{}", config.host, config.port);

    if config.verbose {
        println!("Connecting to {}", addr);
    }

    let stream = TcpStream::connect_timeout(
        &addr.to_socket_addrs()?.next().unwrap(),
        Duration::from_secs(config.timeout),
    )?;

    if config.verbose {
        println!("Connected to {}", addr);
    }

    Ok(())
}

fn port_scan(config: &Config) -> Result<()> {
    let (start_port, end_port) = config.port_range.unwrap_or((1, 1000));

    if config.verbose {
        println!("Scanning {}:{}-{}", config.host, start_port, end_port);
    }

    let mut open_ports = Vec::new();

    for port in start_port..=end_port {
        let addr = format!("{}:{}", config.host, port);

        if config.verbose {
            print!("Scanning port {}... ", port);
            io::stdout().flush().unwrap();
        }

        match TcpStream::connect_timeout(
            &addr.to_socket_addrs()?.next().unwrap(),
            Duration::from_millis(100),
        ) {
            Ok(_) => {
                open_ports.push(port);
                if config.verbose {
                    println!("OPEN");
                } else {
                    println!("Port {} is open", port);
                }
            }
            Err(_) => {
                if config.verbose {
                    println!("CLOSED");
                }
            }
        }
    }

    if open_ports.is_empty() {
        println!("No open ports found in range {}-{}", start_port, end_port);
    } else {
        println!("Found {} open ports: {:?}", open_ports.len(), open_ports);
    }

    Ok(())
}
