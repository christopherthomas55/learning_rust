use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream}
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let delimiter = "\r\n";
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("html_src/hello.html").unwrap();
    let length = contents.len();

    let response = 
        format!("{status_line}{delimiter}Content-Length: {length}{delimiter}{delimiter}{contents}");
    println!("Request: {http_request:#?}");
    stream.write_all(response.as_bytes()).unwrap();
}

