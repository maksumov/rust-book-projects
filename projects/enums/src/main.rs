use std::net::{Ipv4Addr, Ipv6Addr};

// An enum is a type that can be exactly one of its named variants.
// Each variant can hold a different type of data. Using std's
// Ipv4Addr/Ipv6Addr mirrors how std::net::IpAddr is defined.
#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Variants can take any form: no data (Quit), named fields (Move,
// struct-like), a single value (Write), or a tuple (ChangeColor).
// The book's alternative -- four separate structs -- loses here:
// each would be a distinct type, while the enum is a single type
// that a function can accept as one argument.
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn route(ip_addr: IpAddr) {
    println!("Routing {ip_addr:?} IP address!")
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let loopback = IpAddr::V6("::1".parse().expect("invalid Ipv6Addr"));

    route(home);
    route(loopback);
}
