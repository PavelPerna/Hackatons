use std::net::{SocketAddr,UdpSocket,IpAddr};
use crate::core::device::*;
use crate::core::types::string::*;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

pub struct Discovery{
    port:u16,
    devices:RustyDeviceList
}

pub trait Discovering{
    fn discover_configure(&mut self, port: u16);
    fn discover(&mut self) -> !;
    fn discover_stop(&mut self);
}

impl Default for Discovery{
    fn default() -> Discovery{
        Discovery{port:11111,devices:RustyDeviceList::default()}
    }
}


impl Discovering for Discovery{
    
    fn discover_configure(&mut self, port: u16){
        self.port = port;
    }
    fn discover(&mut self) -> !{
        loop{
            // Create a UDP socket
            let socket = UdpSocket::bind("0.0.0.0:11111").expect("Bind socket error"); 
            // Create a buffer to hold incoming
            let mut buf = Box::<[u8]>::from([0;1024]);
            // Receive data from the socket
            let (len, src) = socket.recv_from(&mut buf).expect("Recieve error");
          
            let message = String::from_utf8_lossy(&buf[..len as usize]);
            if message.starts_with("RUSTYNODE"){
                let aip = SocketAddr::from_str(&src.to_string()).expect("Srat");
                let rname = RustyString::from((Box::new(buf),len));
                if self.devices.list.contains_key(&rname) == false{
                    println!("Inserting: {:?}", rname);
                    self.devices.insert(RustyDevice{name:rname,ip:IpAddr::from(aip.ip()),port:aip.port(),name_length:64});
                }
            }
        
            println!("Devices: \r\n");
                //for device in self.devices.clone(){
                    println!("{:?}", self.devices.clone());
                //}
        
            thread::sleep(Duration::from_secs(2));
        }
    }
    fn discover_stop(&mut self){

    }
}
