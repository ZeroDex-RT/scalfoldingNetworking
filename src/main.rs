mod network;
use std::env;


fn main() {

    let args : Vec<String> = env::args().collect();
    println!("{:?}",args);

    if args.len() < 4 {
        eprintln!("Usage : scan host <starting_port> <ending_port>");
    }

    let host = &args[1];

    let start_port : u16 = args[2].parse().expect("Invalid Value");
    let end_port : u16 = args[2].parse().expect("Invalid Value");


    let open_ports = network::scanner::scan_range(host, start_port, end_port);

    println!("Open ports: {:?}", open_ports);
}
