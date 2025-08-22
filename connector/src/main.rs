use crate::udp::UdpServer;
use crate::udp::UdpServer::server_start;

pub mod udp;

fn main() {
    server_start();
}
