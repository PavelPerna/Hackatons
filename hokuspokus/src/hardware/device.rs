pub trait TDevice{
    fn new(name:&str) -> Device;
    fn getName(&self) -> String;
    fn setName(&mut self, val:&str);
}

pub struct Device{
    name:String
}

impl TDevice for Device{

    fn new(name:&str) -> Device{
        Device{
            name:String::from(name)
        }
    }

    fn getName(&self) -> String{
        self.name.clone()
    }

    fn setName(&mut self, val:&str){
        self.name = String::from(val);
    }

}