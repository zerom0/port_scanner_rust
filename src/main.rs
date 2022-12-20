use std::env::args;
use std::net::{SocketAddr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::time::Duration;

mod argument_parser;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 4 {
        panic!("Invalid arguments")
    }

    let ip_address = &args[1];
    let mode = &args[2];
    let port_spec = &args[3];

    let mut ports = Vec::new();
    match mode.as_str() {
        "-s" => {
            let port: u16 = port_spec.parse().expect("Invalid port number");
            ports.push(port);
        }
        "-r" => {
            ports = argument_parser::parse_port_range(port_spec);
        }
        "-l" => {
            ports = argument_parser::parse_port_list(port_spec);
        }
        _ => panic!("Unknown port specification"),
    }
    scan_ports(ip_address, &ports);
}

fn scan_ports(ip_address: &str, ports: &Vec<u16>) {
    for port in ports {
        scan_port(ip_address, port);
    }
}

fn scan_port(ip_address: &str, port: &u16) {
    let address_string = format!("{}:{}", ip_address, port);
    let address_v4 = SocketAddrV4::from_str(&address_string).expect("Invalid address");
    let address = SocketAddr::from(address_v4);
    let timeout = Duration::from_secs(3);

    match TcpStream::connect_timeout(&address, timeout) {
        Ok(_) => println!("Port {} is open ✅", address_v4.port()),
        _ => println!("Port {} is NOT open 🛑", address_v4.port()),
    }
}
