struct IpAddV4 {}

struct IpAddV6 {}

enum IpAddr {
    V4(IpAddV4),
    V6(IpAddV6),
}

fn main() {}