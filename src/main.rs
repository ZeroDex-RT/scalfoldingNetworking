use std::net::{TcpStream,ToSocketAddrs};
use std::time::Duration;

fn check_port(host : &str, port : u16){
    let addr = format!("{}:{}", host,port);

    let timeout = Duration::from_secs(1);
    
    if let Ok(mut addrs) = addr.to_socket_addrs(){
        if let Some(socket_addr) = addrs.next() {
            match TcpStream::connect_timeout(&socket_addr,timeout){
            Ok(_) => println!("[OPEN ] {} ", addr),
            Err(_) => println!("[CLOSED] {}", addr),
            }
        }
    }

}



fn main() {
    let host = "scanme.nmap.org";

    let ports = vec![21,22,80,443,8080];

    for port in ports {
        check_port(host,port);
    }

    println!("\nDone.");
}
