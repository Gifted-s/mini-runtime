// use std::net::{TcpListener, TcpStream};
// use std::io::{Read, Write};
// use std::sync::{Arc, atomic::{AtomicI32, Ordering}};
// use std::thread;

// fn handle_client(mut stream: TcpStream, thread_count: i32) {
//     let mut buffer = [0; 512];
//     if let Ok(_) = stream.read(&mut buffer) {
//         // Increase the thread count atomically
        
//         let response_body = format!("Hello World!! Thread Count: {}", thread_count);
//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//             response_body.len(),
//             response_body
//         );

//         // Write the response to the stream
//         if let Err(e) = stream.write_all(response.as_bytes()) {
//             eprintln!("Failed to write to stream: {}", e);
//         }
        
//         if let Err(e) = stream.flush() {
//             eprintln!("Failed to flush stream: {}", e);
//         }
//     }
// }

// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:7878")?;
//     println!("Server listening on port 7878");

//     // Shared atomic thread counter
//     let mut thread_count = 1;

//     for stream in listener.incoming() {
//         let stream = match stream {
//             Ok(stream) => stream,
//             Err(e) => {
//                 eprintln!("Failed to accept connection: {}", e);
//                 continue;
//             }
//         };

//          thread_count +=1;
       
//         thread::spawn(move || {
//             handle_client(stream, thread_count);
//         });
//     }

//     Ok(())
// }