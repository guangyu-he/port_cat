use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub fn detect_service(stream: &mut TcpStream, _port: u16) -> String {
    stream.set_read_timeout(Some(Duration::from_secs(3))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(3))).ok();

    // 1. 首先尝试读取服务 banner
    if let Some(service) = read_banner(stream) {
        return service;
    }

    // 2. 尝试各种协议探测
    if let Some(service) = probe_http(stream) {
        return service;
    }

    if let Some(service) = probe_database(stream) {
        return service;
    }

    if let Some(service) = probe_mail(stream) {
        return service;
    }

    "Unknown".to_string()
}

fn read_banner(stream: &mut TcpStream) -> Option<String> {
    let mut buffer = [0; 1024];

    if let Ok(n) = stream.read(&mut buffer) {
        if n > 0 {
            let response = String::from_utf8_lossy(&buffer[..n]).to_lowercase();

            if response.contains("ssh-") {
                return Some("SSH".to_string());
            } else if response.contains("220") && response.contains("ftp") {
                return Some("FTP".to_string());
            } else if response.contains("220")
                && (response.contains("smtp") || response.contains("mail"))
            {
                return Some("SMTP".to_string());
            } else if response.contains("* ok") && response.contains("imap") {
                return Some("IMAP".to_string());
            } else if response.contains("+ok") && response.contains("pop") {
                return Some("POP3".to_string());
            } else if response.contains("telnet") || response.contains("login:") {
                return Some("Telnet".to_string());
            }
        }
    }
    None
}

fn probe_http(stream: &mut TcpStream) -> Option<String> {
    let requests = [
        &b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"[..],
        &b"OPTIONS / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"[..],
    ];

    for request in &requests {
        if stream.write_all(request).is_ok() {
            let mut buffer = [0; 1024];
            if let Ok(n) = stream.read(&mut buffer) {
                if n > 0 {
                    let response = String::from_utf8_lossy(&buffer[..n]);
                    if response.starts_with("HTTP/") {
                        // 检查是否是特定的 web 服务器
                        let response_lower = response.to_lowercase();
                        if response_lower.contains("server: nginx") {
                            return Some("HTTP (Nginx)".to_string());
                        } else if response_lower.contains("server: apache") {
                            return Some("HTTP (Apache)".to_string());
                        } else if response_lower.contains("server: microsoft-iis") {
                            return Some("HTTP (IIS)".to_string());
                        } else if response_lower.contains("server: caddy") {
                            return Some("HTTP (Caddy)".to_string());
                        }
                        return Some("HTTP".to_string());
                    } else if response.contains("400 Bad Request") || response.contains("HTTP") {
                        return Some("HTTP".to_string());
                    }
                }
            }
            break;
        }
    }
    None
}

fn probe_database(stream: &mut TcpStream) -> Option<String> {
    // MySQL 握手包检测
    if stream.write_all(b"\x00").is_ok() {
        let mut buffer = [0; 256];
        if let Ok(n) = stream.read(&mut buffer) {
            if n > 4 {
                let response = String::from_utf8_lossy(&buffer[..n]);
                if response.contains("mysql") || buffer[4] == 10 {
                    // MySQL protocol version
                    return Some("MySQL".to_string());
                }
            }
        }
    }

    // PostgreSQL 启动包
    let pg_startup = [
        0x00, 0x00, 0x00, 0x08, // length
        0x04, 0xd2, 0x16, 0x2f, // protocol version
    ];

    if stream.write_all(&pg_startup).is_ok() {
        let mut buffer = [0; 256];
        if let Ok(n) = stream.read(&mut buffer) {
            if n > 0 {
                // PostgreSQL 通常返回认证请求或错误消息
                if buffer[0] == b'R' || buffer[0] == b'E' {
                    return Some("PostgreSQL".to_string());
                }
            }
        }
    }

    // Redis PING 命令
    if stream.write_all(b"*1\r\n$4\r\nPING\r\n").is_ok() {
        let mut buffer = [0; 64];
        if let Ok(n) = stream.read(&mut buffer) {
            if n > 0 {
                let response = String::from_utf8_lossy(&buffer[..n]);
                if response.contains("+PONG") || response.contains("-NOAUTH") {
                    return Some("Redis".to_string());
                }
            }
        }
    }

    // MongoDB 检测 (isMaster 命令)
    let mongo_ping = [
        0x3a, 0x00, 0x00, 0x00, // message length
        0x01, 0x00, 0x00, 0x00, // request id
        0x00, 0x00, 0x00, 0x00, // response to
        0xd4, 0x07, 0x00, 0x00, // opcode (query)
        0x00, 0x00, 0x00, 0x00, // flags
        0x61, 0x64, 0x6d, 0x69, 0x6e, 0x2e, 0x24, 0x63, 0x6d, 0x64, 0x00, // admin.$cmd
        0x00, 0x00, 0x00, 0x00, // skip
        0x01, 0x00, 0x00, 0x00, // return
        0x13, 0x00, 0x00, 0x00, // document length
        0x10, 0x69, 0x73, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, // {ismaster: 1}
    ];

    if stream.write_all(&mongo_ping).is_ok() {
        let mut buffer = [0; 256];
        if let Ok(n) = stream.read(&mut buffer) {
            if n > 16 {
                // MongoDB 响应有特定格式
                let response = String::from_utf8_lossy(&buffer[..n]);
                if response.contains("ismaster") || response.contains("mongodb") {
                    return Some("MongoDB".to_string());
                }
            }
        }
    }

    None
}

fn probe_mail(stream: &mut TcpStream) -> Option<String> {
    // 尝试 SMTP EHLO
    if stream.write_all(b"EHLO localhost\r\n").is_ok() {
        let mut buffer = [0; 512];
        if let Ok(n) = stream.read(&mut buffer) {
            if n > 0 {
                let response = String::from_utf8_lossy(&buffer[..n]);
                if response.contains("250")
                    && (response.contains("smtp") || response.contains("mail"))
                {
                    return Some("SMTP".to_string());
                }
            }
        }
    }

    // 尝试 POP3
    if stream.write_all(b"USER test\r\n").is_ok() {
        let mut buffer = [0; 256];
        if let Ok(n) = stream.read(&mut buffer) {
            if n > 0 {
                let response = String::from_utf8_lossy(&buffer[..n]);
                if response.starts_with("+OK") || response.starts_with("-ERR") {
                    return Some("POP3".to_string());
                }
            }
        }
    }

    // 尝试 IMAP
    if stream.write_all(b"A001 CAPABILITY\r\n").is_ok() {
        let mut buffer = [0; 512];
        if let Ok(n) = stream.read(&mut buffer) {
            if n > 0 {
                let response = String::from_utf8_lossy(&buffer[..n]);
                if response.contains("CAPABILITY") || response.contains("IMAP4") {
                    return Some("IMAP".to_string());
                }
            }
        }
    }

    None
}
