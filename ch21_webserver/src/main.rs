use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};
use trpl;

fn main_async() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        trpl::spawn_task(async move {handle_connection(stream).await});
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let delimiter = "\r\n";
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html_src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            trpl::sleep(Duration::from_millis(5000)).await;
            ("HTTP/1.1 200 OK", "html_src/hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "html_src/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = 
            format!("{status_line}{delimiter}Content-Length: {length}{delimiter}{delimiter}{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    main_async();
}
