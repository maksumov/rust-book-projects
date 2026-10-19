// ch 21: the final project - a multithreaded web server. TCP
// (transport) with HTTP (content) on top, both request-response;
// we work with the raw bytes. Single-threaded first (21.1: listen,
// read, respond, 404, refactor), then the thread pool (21.2) and
// graceful shutdown (21.3). Verification pattern for a server that
// never exits: run in the background, curl 127.0.0.1:7878, stop.

// Listing 21-1: bind to a port (a Result - two servers cannot hold
// one port; unwrap keeps the book's learning-first error policy),
// iterate incoming CONNECTION ATTEMPTS - not connections: the OS
// may refuse some. `_stream` (not the book's `stream`): the bare
// binding warns unused, and the underscore name keeps it clean.
// Expected behavior at this stage: the client gets a connection
// reset (curl exit 56) - nothing is written back yet - while the
// server logs the attempt.
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// Listing 21-2: separating the concerns - accepting the connection
// (main) vs handling it (here). BufReader buffers the stream; the
// request is read line by line until an EMPTY line (a browser ends
// an HTTP request with a blank line - \r\n\r\n). The first line =
// the request line (method GET, URI /, version HTTP/1.1), the rest
// are headers; a GET has no body. The Vec is ignored for now - it
// lives on as request_line in 21-6.
//
// Listings 21-4..21-5: hello.html (project root, not src/) served
// as the body. fs::read_to_string - the minigrep move (ch 12);
// Content-Length in bytes - a valid response must state the body
// size; format! assembles status line + header + CRLF CRLF + body.
// The request is still ignored - ANY path gets this page (fixed in
// 21-6). The 21-3 tiny empty-page response grew into this one.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
