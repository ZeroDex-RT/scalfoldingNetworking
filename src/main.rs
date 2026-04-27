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
   
    let open_ports = network::scanner::scan_range(target,1,1000);

    for port in open_ports {
        println!("[OPEN] : {}", port);
    }
  }
