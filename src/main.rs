mod network;


fn main() {
    let target = "nmap.scanme.org";
    let port = 80;

    let port_open : bool = network::scanner::scan(target,port);
    if port_open {
        println!("[OPEN] : {}:{}",target,port);
    } else {
        println!("[CLOSED] : {}:{}",target,port); 
    }
    
  }
