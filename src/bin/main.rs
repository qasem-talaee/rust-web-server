use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::thread;
use std::time::Duration;

extern crate web_server;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8787").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "web/index.html")
    }
    else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "web/index.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND", "web/404.html")
    };
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}