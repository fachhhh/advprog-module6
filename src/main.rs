use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello_rust::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool: ThreadPool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,  // Successfully read a line
        Some(Err(e)) => {
            eprintln!("Error reading line: {}", e);
            return;
        }
        None => {
            eprintln!("Client disconnected before sending a request.");
            return;
        }
    };    

    /*let (status_line, filename): (&str, &str) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404 Page.html")
    };*/

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404 Page.html"),
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            let error_message = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nInternal Server Error";
            stream.write_all(error_message.as_bytes()).unwrap();
            return;
        }
    };    

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // println!("Request: {:#?}", http_request);
    stream.write_all(response.as_bytes()).unwrap();
}
