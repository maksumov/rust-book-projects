// An enum is a type that can be exactly one of its named variants.
// Each variant can hold a different amount and type of data:
// V4 carries four u8 values, V6 -- a single String.
#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_addr: IpAddr) {
    println!("Routing {ip_addr:?} IP address!")
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);
}
