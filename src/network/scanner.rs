use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;


pub fn scan(host : &str, port : u16) -> bool{
    let addr = format!("{}:{}",host,port);

    let socket_addr = match addr.to_socket_addrs(){
        Ok(mut addrs) => match addrs.next(){
            Some(a) => a,
            None => return false,
        }
        Err(_) => return false,
    };

    let timeout = Duration::from_millis(500);
    TcpStream::connect_timeout(&socket_addr,timeout).is_ok()
}

pub fn scan_range(host : &str, start : u16, end : u16) -> Vec<u16> {
    
    let mut open_ports = Vec::new();

    for port in start..=end {
        if scan(host,port) {
            open_ports.push(port);
        }
    }

    return open_ports;

}
