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

fn main() {
    println!("Hello, world!");
}
