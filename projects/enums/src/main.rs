// An enum is a type that can be exactly one of its named variants.
// Variants can hold data directly (a String here), replacing the
// struct + kind combination from the previous step.
#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn route(ip_addr: IpAddr) {
    println!("Routing {ip_addr:?} IP address!")
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);
}
