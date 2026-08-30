// An enum is a type that can be exactly one of its named variants.
// `#[derive(Debug)]` makes `{ip_kind:?}` print the variant name (V4/V6).
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Combining an enum and a struct works, but is a dead end: `kind` and
// `address` belong together, yet live in separate types. The book's
// next step puts data directly into enum variants, removing the struct.
#[allow(dead_code)]
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing {ip_kind:?} address!")
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    dbg!(&home);
    dbg!(&loopback);
}
