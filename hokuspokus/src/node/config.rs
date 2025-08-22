use crate::model::object::THasConstructor;

pub trait TConfig{
     fn setName(&mut self, name:&str);
     fn setAddress(&mut self, address:&str);
     fn setPort(&mut self, port:u16);
     fn setDelay(&mut self, delay: u16);

     fn getName(&self) -> String;
     fn getAddress(&self) -> String;
     fn getPort(&self) -> u16;
     fn getDelay(&self) -> u16;
}

#[derive(Clone)]
pub struct Config{
    name: String,
    address: String,
    port:u16,
    delay:u16,
}

impl THasConstructor<Config> for Config{
    fn new()->Config{
        Config{
            name:String::from(""),
            address:String::from(""),
            port:0,
            delay:0
        }
    }
}

 impl TConfig for Config{
    fn setName(&mut self, name:&str){
        self.name = String::from(name);
    }
    fn setAddress(&mut self, address:&str){
        self.address = String::from(address);
    }
    fn setPort(&mut self, port:u16){
        self.port = port;
    }
    fn setDelay(&mut self, delay: u16){
        self.delay = delay;
    }

    fn getName(&self) -> String{
        self.name.clone()
    }
    fn getAddress(&self) -> String{
        self.address.clone()
    }
    fn getPort(&self) -> u16{
        self.port
    }
    fn getDelay(&self) -> u16{
        self.delay
    }
}
