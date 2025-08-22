use core::net::{IpAddr,Ipv4Addr};
use std::collections::HashMap;
use std::fmt::{Debug,Formatter,Result};

use crate::core::types::string::*;

//use crate::core::input::*;
//use crate::core::output::*;
#[derive(Clone)]
pub struct RustyDevice{
    pub ip:IpAddr,
    pub port:u16,
    pub name:RustyString,
    pub name_length:u32,
    //input_list:RustyInputList,
    //output_list:RustyOutputList
}

pub type RustyHash<Item> = HashMap<RustyString,Item>;

#[derive(Clone)]
pub struct RustyList<Item>{
    pub list: RustyHash<Item>,
    position: usize,
    key:RustyString,
    len: usize
}

pub type RustyDeviceList = RustyList<RustyDevice>;

impl Default for RustyDevice{
    fn default() -> RustyDevice{
        RustyDevice{ip:IpAddr::V4(Ipv4Addr::new(0,0,0,0)),port:0,name:RustyString::from("Unnamed"),name_length:7}
    }
}

impl Debug for RustyDevice{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        fmt.debug_struct("RustyDevice")
            .field("name", &<RustyString as Into<String>>::into(self.name.clone()))
            .field("ip", &self.ip)
            .field("port", &self.port)
            .finish()
    }
}


impl<T : Debug> Debug for RustyList<T>{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        fmt.debug_list().entries(self.list.iter()).finish()
    }
}
pub trait Identity{
    fn get_id(&self) -> RustyString;
}
impl Identity for RustyDevice{
    fn get_id(&self) -> RustyString{
        RustyString::from(&self.name)
    }
}

impl<Item> Default for RustyList<Item>{
    fn default() -> RustyList<Item>{
        RustyList{
            list:HashMap::new(),
            position:0,
            len:0,
            key:RustyString::from("")
        }
    }
}

impl<Item> RustyList<Item>
where Item:Identity{
    pub fn insert(&mut self,item: Item){
        self.list.insert(item.get_id(),item);
        self.len +=1;
    }

    pub fn get_id(item:Item) -> RustyString{
        item.get_id()
    }
}

pub struct RustyListIterator<T: Clone + Debug>{
    list: RustyList<T>
}

impl<T:Clone + Debug> IntoIterator for RustyList<T>{
    type Item = T;
    type IntoIter = RustyListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter{list:self}
    }
}

impl<T:Clone + Debug> Iterator for RustyListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i : usize = 0;
        for key in self.list.list.clone().into_keys(){
            if i == self.list.position{
                self.list.key = key.clone();
                self.list.position += 1;
                let result = Some(self.list.list.get(&key).unwrap().clone());
                return result;
            }
            i += 1;
        }
        None
    }
}
