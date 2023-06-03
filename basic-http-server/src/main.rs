use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct RequestLine {
    get: &'static str,
}

struct HttpResponse {
    ok: &'static str,
    not_found: &'static str,
}

const HTTP_RESPONSE: HttpResponse = HttpResponse {
    ok: "HTTP/1.1 200 OK",
    not_found: "HTTP/1.1 404 NOT FOUND",
};

const REQUEST_LINE: RequestLine = RequestLine {
    get: "GET / HTTP/1.1",
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

    // Reads request metadata => return lines of string(request metadata)
    // and then map the result and return a Vec.

    // prev implementation.
    /* let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|request| !request.is_empty())
    .collect(); */
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    /* http response have the following
    format:- HTTP-Version Status-Code Reason-Phrase CRLF
             headers CRLF
             message-body */
    let (status_line, filename) = if request_line == REQUEST_LINE.get {
        (HTTP_RESPONSE.ok, "hello.html")
    } else {
        (HTTP_RESPONSE.not_found, "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let content_len = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap()
}
