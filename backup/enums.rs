#![allow(dead_code)]

// ENUMS Option<T>

use std::option;

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 { None } else { Some(value[4]) }
}

fn handle_option(my_options: Vec<Option<i32>>) {
    for item in my_options {
        match item {
            Some(number) => println!("Got a {}", number),
            None => println!("The vec is too short"),
        }
    }
}

fn main() -> Result<(), String> {
    let bigger = vec![1, 2, 3, 4, 5];
    let new_vec = vec![1, 2];

    //     // println!("{:?}, {:?}", take_fifth(new_vec), take_fifth(bigger).unwrap());
    //     match take_fifth(new_vec) {
    //         Some(number) => println!("Number is {}", number),
    //         None => println!("Need more than 5 elements"),
    //     }
    let mut option_vec = Vec::new();
    option_vec.push(take_fifth(new_vec));
    option_vec.push(take_fifth(bigger));

    handle_option(option_vec);

    Ok(())
}
