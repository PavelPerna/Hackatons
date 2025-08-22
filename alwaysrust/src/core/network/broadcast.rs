use core::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use crate::core::types::string::*;

pub trait BroadCasting{
    fn broadcast_configure(&mut self, address:IpAddr, port: u16, content: RustyString, length: u32);
    fn broadcast(&mut self) -> !;
    fn broadcast_stop(&mut self);
}

#[derive(Debug)]
pub struct BroadCast{
    address: IpAddr,
    port: u16,
    content: RustyString,
    length: u32,
    socket: UdpSocket
}

impl Default for BroadCast{
    fn default() -> BroadCast{
        BroadCast{address:IpAddr::V4(Ipv4Addr::new(255,255,255,255)),port:11111,content:RustyString::from("RUSTYNODE-PicaKunda"),length:18,socket:UdpSocket::bind("0.0.0.0:0").expect("Unable to bind socket")}
    }
}

impl BroadCasting for BroadCast{
    fn broadcast_configure(&mut self, address:IpAddr, port: u16, content: RustyString, length: u32){
    
        self.address = address.clone();
        self.port = port;
        self.content = content;
        self.length = length;
    }

    fn broadcast(&mut self) -> !{
        // Broadcast thread
        self.socket.set_broadcast(true).expect("Unable to set broadcast");
        // Send data to a remote address
        let remote_addr = SocketAddr::new(self.address.clone(), self.port);
        loop{
            self.socket.send_to(<RustyString as Into<String>>::into(self.content.clone()).as_bytes(), &remote_addr).expect("Unable to broadcast message");
            thread::sleep(Duration::from_secs(2));
        }
    }

    fn broadcast_stop(&mut self){
        let _ = self.socket.set_broadcast(false).expect("Unable to stop broadcast");
    }
}