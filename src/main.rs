//https://petal-estimate-4e9.notion.site/Week-1-Rust-Meta-programming-22b7dfd107358036bbb4d9467339120d

macro_rules! eval { //Declarative macros!
    {$expr: expr} => {
        $expr
    };
}

macro_rules! vector {
    ($($x: expr ),*)=> {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

use std::fmt::Error; //Non macro soln for serialize & deserializing

trait Serialize{
    fn serialize(&self)-> Vec<u8>;
}

trait Desearialize{
    fn desearialize(v: &[u8]) -> Result<Swap, Error>;
}

#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
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



fn main() {
    let ans = eval!(2 * 3 + 5);
    println!("{ans}");
    let v = vector!(1,2,5);
    println!("{:?}", v);

    let s = Swap{
        qty_1: 1,
        qty_2: 2, 
    };
    let v = s.serialize();
    println!("{:?}", v);

    let s2 = Swap::desearialize(&v).unwrap();
    println!("{:?}", s2);
}
