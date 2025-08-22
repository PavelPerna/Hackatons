pub mod core;

use crate::core::network::broadcast::*;
use crate::core::network::discovery::*;
use std::thread;

fn main() {
    thread::spawn(move ||{
        let mut broadcast = BroadCast::default();
        broadcast.broadcast();
    });
    thread::spawn(move ||{
        let mut discovery = Discovery::default();
        discovery.discover();
    });
    loop{}
}
