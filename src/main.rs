use std::env::args;
use std::net::{SocketAddr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::time::Duration;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 4 || args[2] != "-s" {
        panic!("Invalid arguments")
    }

    let ip_address = &args[1];
    let port: u16 = args[3].parse().expect("Invalid port number");

    scan_port(ip_address, port)
}

fn scan_port(ip_address: &str, port: u16) {
    let address_string = format!("{}:{}", ip_address, port);
    let address_v4 = SocketAddrV4::from_str(&address_string).expect("Invalid address");
    let address = SocketAddr::from(address_v4);
    let timeout = Duration::from_secs(3);

    match TcpStream::connect_timeout(&address, timeout) {
        Ok(_) => println!("Port {} is open ✅", address_v4.port()),
        _ => println!("Port {} is NOT open 🛑", address_v4.port()),
    }
}
