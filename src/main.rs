use clap::Parser;
use std::net::{SocketAddr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::thread::JoinHandle;
use std::time::Duration;
mod argument_parser;

#[derive(Parser)]
struct CLI {
    /// IP address to scan
    address: String,
    /// scan single port, e.g. 80
    #[arg(short)]
    single: Option<String>,
    /// scan port range, e.g. 20-25
    #[arg(short)]
    range: Option<String>,
    /// scan port list, e.g. 22,25,80,443
    #[arg(short)]
    list: Option<String>,
}

fn main() {
    let cli = CLI::parse();

    let ports = if let Some(port_spec) = cli.single {
        argument_parser::parse_single_port(&port_spec)
    } else if let Some(port_spec) = cli.range {
        argument_parser::parse_port_range(&port_spec)
    } else if let Some(port_spec) = cli.list {
        argument_parser::parse_port_list(&port_spec)
    } else {
        panic!("Unknown port specification")
    };

    scan_ports(&cli.address, &ports);
}

fn scan_ports(ip_address: &str, ports: &Vec<u16>) {
    let mut joinhandles = Vec::new();

    for port in ports {
        joinhandles.push(scan_port(ip_address, port));
    }

    for joinhandle in joinhandles {
        joinhandle.join().unwrap();
    }
}

fn scan_port(ip_address: &str, port: &u16) -> JoinHandle<()> {
    let address_string = format!("{}:{}", ip_address, port);
    let address_v4 = SocketAddrV4::from_str(&address_string).expect("Invalid address");
    let address = SocketAddr::from(address_v4);
    let timeout = Duration::from_secs(3);

    std::thread::spawn(
        move || match TcpStream::connect_timeout(&address, timeout) {
            Ok(_) => println!("Port {} is open ✅", address_v4.port()),
            _ => println!("Port {} is NOT open 🛑", address_v4.port()),
        },
    )
}
