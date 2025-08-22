use std::process::Command;
use std::{time::{Duration},thread,net::{UdpSocket}};
use std::io::Write;
use std::io;

use crate::model::object::THasConstructor;
use crate::node::node::*;
use crate::node::config::*;

pub mod hardware;
pub mod node;
pub mod model;

fn main() {
    let mut cfg = Config::new();
    
    cfg.setName("Rustator");
    cfg.setAddress("255.255.255.255");
    cfg.setPort(12345);
    cfg.setDelay(5);
    
    let node = Node{config:cfg};
    
    // Create UDP socket for broadcasting
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to socket");
    socket.set_broadcast(true).expect("Failed to set broadcast option");



    // Create thread for refreshing hosts in network
    thread::spawn(move || {
        loop {
            // Construct the nmap command
            let output = Command::new("arp")
                .output()
                .expect("Failed to execute nmap command");

            let stdout = io::stdout(); 
            let mut lock = stdout.lock();
            
            // Check if the command executed successfully
            if output.status.success() {
                // Parse the nmap output (you might need to adjust this based on nmap's output format)
                let output_str = String::from_utf8_lossy(&output.stdout); 
                // Extract IP addresses from the output (this is a simplified example)
                // You'll likely need more robust parsing logic
                let mut line_nr : usize = 0;
                let mut header : Vec<String> = vec![];
                let mut index : usize = 0;
                    
                for line in output_str.lines() {
                    index = 0;
                    let data = line.split_whitespace().into_iter();
                    for column in data{
                        match line_nr{
                            0 => {header.push(column.to_string());},
                            _ =>{writeln!(&mut lock,"{}:{}",header[index],column);index += 1;}
                        };
                    }
                    writeln!(&mut lock,"\r\n{}\r\n","-".repeat(30));
                    line_nr += 1;
                }
            } else {
            }
            thread::sleep(Duration::from_secs(5)); // Broadcast every 5 seconds (adjust as needed)
        }
    });
    thread::sleep(Duration::from_secs(120));
}