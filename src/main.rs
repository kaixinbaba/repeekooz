#[macro_use]
extern crate log;

fn main() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10));
    println!("{}", localhost_v4);
}
