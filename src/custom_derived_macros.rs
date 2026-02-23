use std::fmt::Error;

trait Serialize{
    fn serialize(&self)-> Vec<u8>;
}

trait Desearialize{
    fn desearialize(v: Vec<u8>) -> swap;
}

struct swap {
    qty_1: u32,
    qty_2: u32,
}

impl Serialize for swap {
    fn serialize(&self)-> Vec<u8>{
        let v = vec![];
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        v
    }
}

impl Desearialize for swap {
     fn desearialize(v: Vec<u8>)-> Result<swap, Error> {

        if v.len() < 8 {
            return Err(Error);
        }
        let qty_1 = u32::from_be_bytes([v[0], v[1], v[2], v[3]]);
        let qty_2 = u32::from_be_bytes([v[4], v[5], v[6], v[7]]);

        return Ok(swap {
            qty_1, 
            qty_2,
        })
     }
}

fn main(){

}