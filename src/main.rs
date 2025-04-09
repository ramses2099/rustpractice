#![allow(dead_code)]

// GENERICS (in functions)
// generic = can be one type, can be another type
// concrete = is one type
// angle brackets <T>
// T, U, V - one capital letter

use std::fmt::{ Debug, Display };
use std::cmp::{ PartialOrd };

// fn return_number(number: i32) -> i32{
//     println!("Here is the return number {}", number);
//     number
// }
fn return_number<T>(number: T) -> T {
    println!("Here is the thing");
    number
}

fn print_number<T: Display>(number: T) {
    println!("Here is your number: {}", number);
}

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item {:?}", item)
}

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!("{}! is {} greated than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

fn main() -> Result<(), String> {
    let n = return_number("String".to_string());
    println!("{}", n);
    //
    print_number(32);

    let charlie = Animal {
        name: String::from("Charlie"),
        age: 1,
    };
    //
    print_item(charlie);

    //
    compare_and_display("test for compare".to_string(), 10, 12);

    Ok(())
}
