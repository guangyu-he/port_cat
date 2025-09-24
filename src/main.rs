mod connect;
mod detect_service;
mod scan;

use clap::Parser;
use env_logger::Env;
use log::error;

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
    #[clap(short, long)]
    scan: Option<String>,

    /// Timeout in seconds
    #[clap(short, long, default_value_t = 5)]
    timeout: u64,

    /// Verbose output
    #[clap(long, default_value = "info")]
    debug_level: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(args.debug_level.as_str()))
        .format_timestamp_secs()
        .init();

    match if args.scan.is_some() {
        scan::scan_mode(args).await
    } else {
        connect::connect_mode(args)
    } {
        Ok(_) => {}
        Err(e) => {
            error!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
