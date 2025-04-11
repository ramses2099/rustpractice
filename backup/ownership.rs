#![allow(dead_code)]

fn calculate_length(s: &String) -> usize {
    let len = s.len();
    len
}

fn main() -> Result<(), String> {
    // ----- Ownership rules ------//
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
    // The Rules of References
    // 1. At any given time, you can have either one mutable reference
    // or any number of immutable references.
    //
    // 2. References must alway be valid.
    

    Ok(())
}
