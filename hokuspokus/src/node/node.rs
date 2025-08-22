use crate::node::config::{Config,TConfig};
use std::time::Duration;
use std::thread;
use std::net::{SocketAddr,UdpSocket};

pub struct NodeInfo{
    name:String,
    ip_address:String
}

pub trait TNode{
    async fn broadcast_id(&self);
    fn node_info(&self) -> NodeInfo;
}

pub struct Node{
    pub config:Config
}

impl TNode for Node{
    async fn broadcast_id(&self){
        // Create UDP socket for broadcasting
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to socket");
        socket.set_broadcast(true).expect("Failed to set broadcast option");
        let broadcast_message = self.config.getName(); // Replace with your application identifier
        let broadcast_address = self.config.getAddress();
        let broadcast_port = self.config.getPort();
        let broadcast_delay = self.config.getDelay();
        
        // Create thread for periodic broadcast
        thread::spawn(move || {
            loop {
                socket.send_to(broadcast_message.as_bytes(), &SocketAddr::new(broadcast_address.parse().expect("IP suck"), broadcast_port))
                    .expect("Couldn't send broadcast");
                thread::sleep(Duration::from_secs(broadcast_delay.into())); // Broadcast every 5 seconds (adjust as needed)
            }
        });
    }

    fn node_info(&self) ->NodeInfo{
        NodeInfo{ip_address:self.config.getAddress().clone(),name:self.config.getName().clone()}
    }
}
