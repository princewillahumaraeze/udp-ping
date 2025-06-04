use std::arch::x86_64::_SIDD_CMP_EQUAL_ANY;
use std::net::{UdpSocket, SocketAddr};
use std::time::{Duration, Instant};
use std::env;
use std::io::{self, ErrorKind};

//Payload prefix to identify our packet
const PAYLOAD_PREFIX: &[u8] = b"RUST_UDP_PING"; 
const DEFAULT_TIMEOUT_SECONDS :u64 = 2; //Default timeout for reply
const DEFAULT_PAYLOAD: &str = "Default Echo";

struct PacketConfig{
    target_addr: SocketAddr,
    payload: String,
    source_ip: String,
}

fn print_usage(program_name: &str){
    eprintln!("Usage: {} <target_ip:port> [optional_payload] 
                [--source <local_ip_address>]", program_name);
    eprintln!("Example: {} 8.8.8.8:53 \"Test Echo\" --source 192.168.1.100",
                program_name);
    eprintln!("Example: {} 127.0.0.1:7878", program_name);
    eprintln!("Notes:");
    eprintln!(" - The target must be running a UDP echo service
                or a service that might echo unexpected UDP packets.");
    eprintln!(" - If --source is not provided,
                it defaults to 0.0.0.0 (all interfaces).");
    eprintln!(" - The local port for the source is always chosen by the OS
                (e.g., <local_ip_address>:0).");
}

fn parse_arguments(args: Vec<String>) -> Result<PacketConfig, io::Error>{
    unimplemented!()
}

fn create_socket(local_ip_str: &str) -> io::Result<UdpSocket>{
    unimplemented!()
}

fn build_socket_payload(sequence_payload: u32,
     custom_payload_str:&str
)->Vec<u8>{
    unimplemented!()
}

fn perform_echo_ping(socket: &UdpSocket,
    target_addr:SocketAddr,
    sequence_number:u32,
    payload_str:&str
) -> io::Result<()>{
    unimplemented!()
}

fn main() -> io::Result<()>{
    // Argument parsing
    let config = match parse_arguments(env::args().collect()){
        Ok(cfg) => cfg,
        Err(e) => {
            print_usage(&env::args().next().unwrap_or_else(|| "udp_ping".to_string()));
            return Err(e);
        }
    };

    println!("UDP Ping Utility");
    println!("Target: {}, Payload: \"{}\", Source Ip: {}",
     config.target_addr, config.payload, config.source_ip);

    // Socket creation
    let socket = match create_socket(&config.source_ip) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Could not create or bind sockets: {} ", e);
            return Err(e);
        }
    };
    println!("Bound to local address: {}", socket.local_addr()?);

    // Packet Operations
    let sequence_number: u32 = 1;
    perform_echo_ping(&socket,
         config.target_addr, sequence_number, &config.payload);

}
