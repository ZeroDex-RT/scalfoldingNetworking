use std::net::{TcpStream,ToSocketAddrs};
use std::time::Duration;
use std::thread;

pub fn scan(host : &str, port : u16) -> bool {

    let addr = format!("{}:{}", host,port);

    let socket_addr = match addr.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next(){
            Some(addr) => addr,
            None => return false,
        }
        Err(_) => return false,
    };
    

    let timeout = Duration::from_millis(500);

    TcpStream::connect_timeout(&socket_addr,timeout).is_ok()

}

pub fn scan_range(host : &str, start : u16, end : u16) -> Vec<u16> {

    let mut handles = Vec::new();

    for port in start..=end {
        let host = host.to_string();

        let handle = thread::spawn(move || {
            if scan(&host,port){
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
