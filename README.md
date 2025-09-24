# Port Cat 🐱

A fast and efficient CLI tool for testing network connectivity and scanning ports on remote hosts, written in Rust.

## Features

- **Connection Testing**: Test connectivity to specific ports
- **Port Scanning**: Scan port ranges with concurrent execution
- **Service Detection**: Automatically detect running services (HTTP, SSH, databases, etc.)
- **Async Performance**: High-performance concurrent scanning using Tokio
- **Detailed Logging**: Configurable log levels for debugging

## Installation

### From Source

```bash
git clone https://github.com/yourusername/port_cat.git
cd port_cat
cargo build --release
```

The binary will be available at `target/release/port_cat`

### Using Cargo

```bash
cargo install --path .
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

### Service Detection

The tool automatically detects services running on open ports:

```
Connected to example.com:22 - Service: SSH
Connected to example.com:80 - Service: HTTP (Nginx)
Connected to example.com:443 - Service: HTTPS
Connected to example.com:3306 - Service: MySQL
```

Supported service detection:

- **Web Services**: HTTP, HTTPS (with server detection: Nginx, Apache, IIS, Caddy)
- **SSH**: Secure Shell
- **Databases**: MySQL, PostgreSQL, Redis, MongoDB
- **Mail Services**: SMTP, POP3, IMAP
- **FTP**: File Transfer Protocol
- **And more...**

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
3. **Response Analysis**: Analyzes server responses to identify specific services and versions

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
