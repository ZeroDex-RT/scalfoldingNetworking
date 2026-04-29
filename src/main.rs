mod network;
use std::env;
use std::net::ToSocketAddrs;


fn main() {

    let args : Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage : scan host <starting_port> <ending_port>");
        return;
    }

    let host = &args[1];
    let start_port : u16 = args[2].parse().expect("Invalid Value");

    let socket_addr = match (host.as_str(),start_port).to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr,
            None => {
                eprintln!("Could not resolve host");
                return;
            },
        }
        Err(_) => {
            eprintln!("failed DNS Resolution");
            return;
        }
    };
    


    if args.len() == 3 {
        let port_open = network::scanner::scan(socket_addr);
        if port_open {
            println!("port {} is open",start_port);
        }
        return;

    }

    let end_port : u16 = args[3].parse().expect("Invalid Value");


    let open_ports = network::scanner::scan_range(socket_addr.ip(), start_port, end_port);

    for port in open_ports {
        println!("port {} is open" , port);
    }
}
