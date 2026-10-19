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
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }
}
