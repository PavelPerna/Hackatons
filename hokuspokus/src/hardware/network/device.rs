pub trait TNetworkDevice{
    fn new(name:&str,ip_address:&str,host_name:&str,adapter:&str,mac_address:&str) -> NetworkDevice;
    fn getName(&self) -> String;
    fn getIpAddress(&self) -> String;
    fn getHostName(&self) -> String;
    fn getAdapter(&self) -> String;
    fn getMacAddress(&self) -> String;

    fn setName(&mut self, val:&str);
    fn setIpAddress(&mut self, val:&str); 
    fn setHostName(&mut self, val:&str);
    fn setAdapter(&mut self, val:&str);
    fn setMacAddress(&mut self, val:&str);
}

pub struct NetworkDevice{
    name:String,
    ip_address:String,
    host_name:String,
    adapter:String,
    mac_address:String
}

impl TNetworkDevice for NetworkDevice{

    fn new(name:&str,ip_address:&str,host_name:&str,adapter:&str,mac_address:&str) -> NetworkDevice{
        NetworkDevice{
            name:String::from(name),
            ip_address:String::from(ip_address),
            host_name:String::from(host_name),
            adapter:String::from(adapter),
            mac_address:String::from(mac_address)
        }
    }

    fn getName(&self) -> String{
        self.name.clone()
    }
    fn getIpAddress(&self) -> String{
        self.ip_address.clone()
    }
    fn getHostName(&self) -> String{
        self.host_name.clone()
    }
    fn getAdapter(&self) -> String{
        self.adapter.clone()
    }
    fn getMacAddress(&self) -> String{
        self.mac_address.clone()
    }

    fn setName(&mut self, val:&str){
        self.name = String::from(val);
    }
    fn setIpAddress(&mut self, val:&str){
        self.ip_address = String::from(val);
    } 
    fn setHostName(&mut self, val:&str){
        self.host_name = String::from(val);
    }
    fn setAdapter(&mut self, val:&str){
        self.adapter = String::from(val);
    }
    fn setMacAddress(&mut self, val:&str){
        self.mac_address = String::from(val);
    }

}