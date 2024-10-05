use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::{fs, thread};
use std::time::Duration;

use multi_thread_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

// reading request and sending content
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024]; // store a buffer thats 1024 bytes long
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = 
    if buffer.starts_with(get){
        ("HTTP/1.1 200 OK", "index.html")
    }else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "index.html")
    } else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}