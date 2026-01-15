use clap::Parser;
use env_logger::Env;
use log::error;

use port_cat::Args;
use port_cat::connect;
use port_cat::scan;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(args.debug_level.as_str()))
        .format_timestamp_secs()
        .init();

    match if args.scan.is_some() {
        scan::scan_mode(args).await
    } else {
        connect::connect_mode_cli(args)
    } {
        Ok(_) => {}
        Err(e) => {
            error!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
