# Port Cat 🐱

A fast Rust CLI and PyO3-powered Python extension for testing connectivity, scanning port ranges, and detecting common services.

## Features

- **Connection Testing**: Test connectivity to specific ports
- **Port Scanning**: Scan port ranges with concurrent execution
- **Service Detection**: Detect HTTP, SSH, FTP, SMTP, POP3, IMAP, Telnet, MySQL, PostgreSQL, Redis, and MongoDB
- **Python Bindings**: `connect_mode` (sync) and `scan_mode` (async) exposed via PyO3
- **Rich Connection Info**: Connection details including buffers, keepalive, and remote address
- **Detailed Logging**: Configurable log levels for debugging

## Installation

### Rust CLI (from source)

```bash
git clone https://github.com/guangyu-he/port_cat.git
cd port_cat
cargo build --release
```

The binary will be available at `target/release/port_cat`

### Using Cargo

```bash
cargo install --path .
```

### Python bindings (from source)

With Python 3.10+ and maturin installed:

```bash
maturin develop --features python
```

## Usage

### Basic Connection Testing

Test connectivity to specific ports:

```bash
# Test default ports (80, 443) on localhost
port_cat

# Test specific host and ports
port_cat example.com -p 22,80,443

# Test with custom timeout
port_cat example.com -p 22 -t 10
```

### Port Range Scanning

Scan a range of ports:

```bash
# Scan ports 1-1000
port_cat example.com -s 1-1000

# Scan common ports
port_cat example.com -s 1-65535
```

### Python API

```python
import asyncio
from port_cat import connect_mode, scan_mode

connections = connect_mode(ports=[22, 80], host="example.com", timeout=5)
open_ports = asyncio.run(scan_mode(host="example.com", scan_range="1-1024"))
```

`connect_mode` returns a list of `ConnectionInfo` objects with connection metadata and detected service.

## Command Line Options

```
USAGE:
    port_cat [OPTIONS] [HOST]

ARGS:
    <HOST>    Host to connect to [default: localhost]

OPTIONS:
    -p, --port <PORT>           Port numbers (comma-separated) [default: 80,443]
    -s, --scan <RANGE>          Scan mode with port range (e.g., 1-1000)
    -t, --timeout <TIMEOUT>     Timeout in seconds [default: 5]
        --debug-level <LEVEL>   Log level [default: info] [possible values: error, warn, info, debug, trace]
    -h, --help                  Print help
    -V, --version               Print version
```

## Examples

### Test Web Services

```bash
# Test if a website is accessible
port_cat google.com -p 80,443
```

### Database Connectivity

```bash
# Test database connections
port_cat db.example.com -p 3306,5432,6379
```

### Port Scanning

```bash
# Quick scan of common ports
port_cat target.com -s 1-1024

# Scan with debug output
port_cat target.com -s 20-25 --debug-level debug
```

### SSH Connection Test

```bash
# Test SSH connectivity
port_cat server.example.com -p 22
```

## Performance

Port Cat is designed for speed and efficiency:

- **Concurrent Scanning**: Multiple ports scanned simultaneously using async/await
- **Fast Service Detection**: Protocol-specific probes for accurate service identification
- **Configurable Timeouts**: Adjust timeout values for different network conditions
- **Low Resource Usage**: Efficient memory and CPU utilization

## Service Detection Technology

The tool uses multiple detection methods:

1. **Banner Reading**: Captures service banners automatically sent by servers
2. **Protocol Probing**: Sends protocol-specific requests (HTTP, database handshakes, etc.)
3. **Response Analysis**: Analyzes server responses to identify specific services

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
