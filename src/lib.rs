pub mod connect;
pub mod detect_service;
pub mod scan;

use clap::Parser;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Host to connect to
    #[clap(index = 1, default_value = "localhost")]
    host: String,

    /// Port number
    #[clap(short, long, value_delimiter = ',', default_value = "80,443")]
    port: Vec<u16>,

    /// Scan mode
    #[clap(short, long)]
    pub scan: Option<String>,

    /// Timeout in seconds
    #[clap(short, long, default_value_t = 5)]
    timeout: u64,

    /// Verbose output
    #[clap(long, default_value = "info")]
    pub debug_level: String,
}

#[cfg(feature = "python")]
#[pymodule]
mod port_cat {
    #[pymodule_export]
    use crate::connect::connect_mode;
}
