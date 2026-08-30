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

fn route(ip_addr: IpAddr) {
    println!("Routing {ip_addr:?} IP address!")
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let loopback = IpAddr::V6("::1".parse().expect("invalid Ipv6Addr"));

    route(home);
    route(loopback);
}
