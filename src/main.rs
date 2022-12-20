use std::net::{SocketAddr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::time::Duration;

fn main() {
    let address = SocketAddr::from(SocketAddrV4::from_str("127.0.0.1:80").unwrap());
    let timeout = Duration::from_secs(3);
    let result = TcpStream::connect_timeout(&address, timeout);
    println!("{:?}", result);
}
