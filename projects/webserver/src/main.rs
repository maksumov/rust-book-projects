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
// an HTTP request with a blank line - \r\n\r\n). The collected Vec
// was pretty-printed at this step in the book: first line = the
// request line (method GET, URI /, version HTTP/1.1), the rest are
// headers; a GET has no body. `_http_request`: dead once 21-3
// replaces the print - it lives on as request_line in 21-6.
//
// Listing 21-3: the tiny success response - a status line and
// CRLF CRLF, no headers, no body; write_all sends the bytes down
// the connection. curl now gets an EMPTY 200 page - no body yet.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
