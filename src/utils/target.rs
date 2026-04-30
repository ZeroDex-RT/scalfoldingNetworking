use std::io::Write;
use std::net::TcpListener;
use std::thread;

fn main() {
    let ports = vec![8000, 2222, 3306];

    for port in ports {
        thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let listener = TcpListener::bind(&addr).expect(&format!("Failed to bind to {}", addr));

            println!("Listening on {}", addr);

            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let banner = get_banner(port);
                        let _ = stream.write_all(banner.as_bytes());
                    }
                    Err(e) => {
                        eprintln!("Connection failed: {}", e);
                    }
                }
            }
        });
    }

    // Keep main thread alive
    loop {
        thread::park();
    }
}

fn get_banner(port: u16) -> &'static str {
    match port {
        8000 => "HTTP/1.1 200 OK\r\nServer: TestServer\r\n\r\nHello",
        2222 => "SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.3",
        3306 => "5.7.31-MySQL",
        _ => "Unknown Service",
    }
}
