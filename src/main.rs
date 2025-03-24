
use std::{num::ParseIntError, result::Result};
// use std::option::Option;



fn to_int(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}


fn sum_str_vec(strs:Vec<String>) -> Result<String, ParseIntError>{
    let mut accum = 0;
    for s in strs {
        accum += to_int(&s)?; //nError)?; //
    }
    Ok(accum.to_string())
}


fn main() {
    let v = vec![String::from("3"),String::from("4")];
    let total = sum_str_vec(v);
    println!("{:?}", total);

    let v = vec![String::from("3"),String::from("abc")];
    let total = sum_str_vec(v);
    println!("{:?}", total);
   
}
