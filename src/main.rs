enum IpAddr {
    V4(&'static str),
    V6(&'static str),
}

fn main() {
    let v4 = IpAddr::V4("127.0.0.1");
    let v6 = IpAddr::V6("::1");
}