use std::net::TcpStream;


pub fn scan(host : &str, port : u16) -> bool{
    let addr = format!("{}:{}",host,port);

    match TcpStream::connect(addr){
        Ok(_) => true,
        Err(_) => false,
    }
}
