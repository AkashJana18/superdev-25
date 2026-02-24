//https://petal-estimate-4e9.notion.site/Week-1-Rust-Meta-programming-22b7dfd107358036bbb4d9467339120d
mod custom_derived_macros;
use custom_derived_macros::{Swap, Desearialize, Serialize};

macro_rules! eval { //Declarative macros!

    {  $expr:  expr  } => {

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
