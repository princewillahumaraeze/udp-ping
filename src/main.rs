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
    unimplemented!()
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

}
