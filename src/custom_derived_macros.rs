use std::fmt::Error; //Non macro soln for serialize & deserializing

pub trait Serialize{
     fn serialize(&self)-> Vec<u8>;
}

pub trait Desearialize{
     fn desearialize(v: &[u8]) -> Result<Swap, Error>;
}

#[derive(Debug)]
pub struct Swap {
    pub qty_1: u32,
    pub qty_2: u32,
}

 impl Serialize for Swap { 
     fn serialize(&self)-> Vec<u8>{
        let mut v = Vec::new();
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        v
    }
}

 impl Desearialize for Swap {
      fn desearialize(v: &[u8])-> Result<Swap, Error> {

        if v.len() < 8 {
            return Err(Error);
        }
        let qty_1 = u32::from_be_bytes([v[0], v[1], v[2], v[3]]);
        let qty_2 = u32::from_be_bytes([v[4], v[5], v[6], v[7]]);

        return Ok(Swap {
            qty_1, 
            qty_2,
        })
     }
}