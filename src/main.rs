use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use runtime::Response;

fn handle_connection(mut stream: TcpStream) -> Response {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_root = b"GET / HTTP/1.1\r\n";
    let get_hello = b"GET /hello HTTP/1.1\r\n";

    // let runtime =  runtime::start_runtime();
    let (status_line, content) = if buffer.starts_with(get_root) {
        // Simulate some blocking work
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "Hello, World updated!")
    } else if buffer.starts_with(get_hello) {
        // Simulate some blocking work
        thread::sleep(Duration::from_secs(1));
        ("HTTP/1.1 200 OK\r\n\r\n", "Hello from /hello!")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404 - Not Found")
    };

    let response = format!("{}{}", status_line, content);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    return Response {};
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    println!("Server running on http://127.0.0.1:3000");

    let runtime = runtime::start_runtime();
     
    // event loop :)
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New connection from: {}", addr);
                runtime.schedule(move || handle_connection(stream.try_clone().unwrap()));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // No incoming connection available
                println!("No incoming connection, waiting...");
                thread::sleep(Duration::from_millis(500)); // Simulate waiting
            }
            Err(e) => {
                println!("Error accepting connection: {:?}", e);
            }
        }
    }
}
