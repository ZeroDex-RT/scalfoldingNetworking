mod network;
mod utils;
use std::env;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage : scan host <starting_port> <ending_port>");
        return;
    }

    let host = &args[1];
    let start_port: u16 = args[2].parse().expect("Invalid Value");

    let socket_addr = match (host.as_str(), start_port).to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr,
            None => {
                eprintln!("Could not resolve host");
                return;
            }
        },
        Err(_) => {
            eprintln!("failed DNS Resolution");
            return;
        }
    };

    println!("Scanning {} (IP: {})", host, socket_addr.ip());

    if args.len() == 3 {
        let port_open = network::scanner::scan(socket_addr);
        println!("Port: {}", start_port);
        if port_open {
            println!(
                "[OPEN : {:?}] : {}",
                utils::banner_grabber::grab_banner(socket_addr),
                start_port
            );
        }
        println!("Scanning Completed in {:?}", start.elapsed());
        return;
    }

    let end_port: u16 = args[3].parse().expect("Invalid Value");
    println!("Range {}-{} \n", start_port, end_port);

    let mut open_ports = network::scanner::scan_range(socket_addr.ip(), start_port, end_port);
    open_ports.sort();
    println!("Open Ports:");
    for port in open_ports {
        let addr = SocketAddr::new(socket_addr.ip(), port);

        let banner = utils::banner_grabber::grab_banner(addr);
        let service = utils::banner_grabber::detect_service(port, banner.as_deref());

        print!("- {} | {}", port, service);

        if port == 443 {
            println!(" | HTTPS (encrypted)");
            continue;
        }

        if let Some(ref b) = banner {
            let mut lines = b.lines();

            if let Some(status_line) = lines.next() {
                print!(" | {}", status_line);

                for line in lines {
                    if line.to_lowercase().starts_with("server:") {
                        print!(" | {}", line);
                        break;
                    }
                }
            }
        }

        println!();
    }

    println!("Scanning Completed in {:?}", start.elapsed());
}
