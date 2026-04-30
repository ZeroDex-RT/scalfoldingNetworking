use std::io::{Read, Write};
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

pub fn grab_banner(addr: SocketAddr) -> Option<String> {
    let timeout = Duration::from_secs(2);

    if let Ok(mut stream) = TcpStream::connect_timeout(&addr, timeout) {
        stream.set_read_timeout(Some(timeout)).ok()?;

        let mut buffer = [0; 1024];

        if let Ok(n) = stream.read(&mut buffer) {
            if n > 0 {
                return Some(String::from_utf8_lossy(&buffer[..n]).to_string());
            }
        }

        
        let request = b"GET / HTTP/1.0\r\n\r\n";
        stream.write_all(request).ok()?;

        if let Ok(n) = stream.read(&mut buffer) {
            if n > 0 {
                return Some(String::from_utf8_lossy(&buffer[..n]).to_string());
            }
        }
    }

    None
}

pub fn detect_service(port: u16, banner: Option<&str>) -> String {
    match port {
        80 | 8080 => "HTTP".to_string(),
        443 => "HTTPS".to_string(),
        22 => "SSH".to_string(),
        21 => "FTP".to_string(),
        25 => "SMTP".to_string(),
        _ => {
            if let Some(b) = banner {
                if b.contains("SSH") {
                    return "SSH".to_string();
                }
                if b.contains("HTTP") {
                    return "HTTP".to_string();
                }
            }
            "Unknown".to_string()
        }
    }
}
