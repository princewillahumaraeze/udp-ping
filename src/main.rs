use std::net::{UdpSocket, SocketAddr};
use std::time::{Duration, Instant};
use std::env::{self, args};
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
    if args.len() < 2 {
        // i.e Program Name + target_ip:port is minimum
        return Err(io::Error::new(ErrorKind::InvalidInput, 
            "Too few arguments"));
    }

    let mut target_addr_str: Option<String> = None;
    let mut payload_str= DEFAULT_PAYLOAD.to_string();
    let mut source_ip_str  = "0.0.0.0".to_string();

    let mut iter = args.iter().skip(1);

    // Check if the first argument is an IP address
    if let Some(target_arg) = iter.next(){
        if target_arg.starts_with("--"){
            return Err(io::Error::new(ErrorKind::InvalidInput,
                 "target address must be the first argument"));
        }
        target_addr_str = Some(target_arg.clone());
    } else {
        return Err(io::Error::new(ErrorKind::InvalidInput,
                 "Missing target address"));
    }

    // Process remaining optional arguments
    while let Some(arg) = iter.next() {
        if arg == "--source" {
            if let Some(ip_val) = iter.next(){
                // Perform validation to be sure it an ip address
                if ip_val.parse::<std::net::IpAddr>().is_err(){
                    return Err(io::Error::new(ErrorKind::InvalidInput,
                        format!("Invalid format format for --source: {}",
                         ip_val)
                        )
                    );
                }
                source_ip_str = ip_val.clone();
            } else {
                return Err(io::Error::new(ErrorKind::InvalidInput,
                 "Missing value for the '--source' argument"));
            }
        } else if !arg.starts_with("--") {
            payload_str = arg.clone()
        } else {
            return Err(io::Error::new(ErrorKind::InvalidInput,
                format!("Unknown option or argument: {}",arg)));
        }
    }

    let target_addr: SocketAddr = match target_addr_str.unwrap().parse() {
        Ok(addr) => addr,
        Err(_) => {
            return Err(io::Error::new(ErrorKind::InvalidInput, 
                "Invlid target address format. Use IP:Port"));
        }
    };

    Ok(PacketConfig{
        target_addr,
        payload: payload_str,
        source_ip: source_ip_str
    })
}

fn create_socket(local_ip_str: &str) -> io::Result<UdpSocket>{
    // Bind to the specified local_ip and let the os pick the available port
    let bind_addr_str = format!("{}:0", local_ip_str);

    match UdpSocket::bind(&bind_addr_str){
        Ok(s) => Ok(s),
        Err(e) => {
            eprintln!("Error: Could not bind to local address: {}: {}",
             bind_addr_str, e);
            Err(e)
        }
    }
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
