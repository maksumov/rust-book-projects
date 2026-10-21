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
    thread,
    time::Duration,
};

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // Listing 21-25: two requests served, then a graceful shutdown -
    // take(2) ends the iterator, main's scope ends, the pool's Drop
    // runs: the sender dies (21-23), the workers break their loops
    // (21-24), join collects them (21-22). The third request gets a
    // refusal. A demo-only device - a real server never exits.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // Listing 21-12: the ideal interface FIRST - ThreadPool::new(4)
        // + pool.execute, deliberately shaped like thread::spawn (a
        // drop-in swap). Compiler-driven development: this state does
        // NOT compile - E0433 (cannot find type `ThreadPool`) is the
        // first instruction: give the crate a lib (21-11's spawn was
        // replaced by execute; its story lives in the git history).
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
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
//
// Listings 21-6..21-8: routing - the request line alone decides:
// only a well-formed GET / gets the page, anything else answers
// 404 NOT FOUND with 404.html. The iterator is bound ONCE (a
// deviation from the book's lines().next() one-liner): the first
// line routes, the remaining lines until the empty one collect as
// headers - alive only for the print switch below.
//
// Listing 21-9: the refactor - the branches differed only in the
// status line and the filename; the tuple + destructuring let (a
// ch 19 pattern) hoists that difference, and the read/format/
// write path becomes single-copy.
//
// Listing 21-10: the slow-request simulation - GET /sleep sleeps
// 5s before answering. The if became a MATCH: three arms now, and
// match needs the slice (&request_line[..]) - it does not
// auto-reference like ==. The observable this stage exists FOR:
// while /sleep holds the ONLY thread, a concurrent GET / waits out
// the whole sleep (verified: ~4s) - the pain the thread pool
// (21-11+) is about to fix.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let mut lines = buf_reader.lines();

    let request_line = lines.next().unwrap().unwrap();
    let _headers: Vec<_> = lines
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // Uncomment to watch the request line and the headers arrive
    // in the server console (listing 21-2's observable, kept
    // switchable at the 21-6+ stage):
    // println!("Request line: {request_line}\nHeaders: {_headers:#?}");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
