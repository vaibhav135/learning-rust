use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let base_url: &str = "127.0.0.1";
    let port: &str = "7878";
    let url: String = format!("{}:{}", base_url, port);

    let listener = TcpListener::bind(url).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|request| !request.is_empty())
        .collect();

    println!("{:#?}", http_request);
}
