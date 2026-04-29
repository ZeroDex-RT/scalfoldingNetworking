use std::net::{TcpStream,SocketAddr};
use std::time::Duration;
use std::thread;

pub fn scan(addr : SocketAddr) -> bool {    
    let timeout = Duration::from_millis(500);
    TcpStream::connect_timeout(&addr,timeout).is_ok()
}

pub fn scan_range(base_ip: std::net::IpAddr, start : u16, end : u16) -> Vec<u16> {

    let mut handles = Vec::new();

    for port in start..=end {
        let addr = SocketAddr::new(base_ip,port);

        let handle = thread::spawn(move || {
            if scan(addr){
                Some(port)
            } else {
                None
            }
        });

        handles.push(handle);
    }

    let mut open_ports = Vec::new();

    for handle in handles {
        if let Ok(Some(port)) = handle.join() {
            open_ports.push(port);
        }
    }

    return open_ports;
}
