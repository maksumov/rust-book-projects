// An enum is a type that can be exactly one of its named variants.
// `#[derive(Debug)]` makes `{ip_kind:?}` print the variant name (V4/V6).
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing {ip_kind:?} address!")
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}
