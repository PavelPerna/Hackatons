use std::borrow::Cow;
use std::fmt::{Result,Formatter,Debug};

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct RustyString{
    buf:Box<[u8]>,
    len:u16
}

impl Default for RustyString{
    fn default() -> RustyString{
        RustyString{buf:Box::new([0]),len:0}
    }
}

impl Debug for RustyString{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        fmt.debug_struct("RustyString")
            .field("val",&<RustyString as Into<String>>::into(self.clone()))
            .finish()
    }
}

impl Into<String> for RustyString{
    fn into(self) -> String{
        String::from_utf8_lossy(&self.buf[0..self.len as usize]).to_string()
    }
}

impl<'a> Into<Cow<'a,str>> for RustyString{
     fn into(self) -> Cow<'a,str>{
        let binding :String = self.into();
        binding.into()
    }
}

impl Into<[u8;1024]> for RustyString{
    fn into(self) -> [u8;1024]{
       let mut buffer : [u8;1024] = [0;1024];
       let i = 0;
       for n in self.buf{
            buffer[i] = n;
       }
       buffer
   }
}

impl From<String> for RustyString{
    fn from(s:String) -> Self{
        RustyString{buf:s.clone().into_boxed_str().into(),len:s.len() as u16}
    }
}
impl From<&str> for RustyString{
    fn from(s:&str) -> Self{
        RustyString{buf:String::from(s).into_boxed_str().into(),len:s.len() as u16}
    }
}
impl From<(Box<Box<[u8]>>, usize)> for RustyString{
    fn from(b:(Box<Box<[u8]>>, usize)) -> Self{
        RustyString{buf:*b.0, len: b.1 as u16}
    }
}

impl From<[u8;1024]> for RustyString{
    fn from(b:[u8;1024]) -> Self{
        let mut length: usize = 0;
        for v in b{
            if v == 0{
                break;
            }
            length += 1;
        }
        let buffer = String::from_utf8_lossy(&b[0..length]);
        RustyString{buf:buffer.to_string().into_boxed_str().into(), len:1024 }
    }
}

impl From<&RustyString> for RustyString{
    fn from(b:&RustyString) -> Self{
        b.clone()
    }
}