mod mod02;
use mod02::{Inches, Seconds, Centimeters};

fn main() {
    let _one_second = Seconds(1);
    // Error: `Seconds` can't be printed; it doesn't implement
    // the Debug trait
    // println!("{:?} seconds", _one_second);
    // Error: `Seconds` can't be printed; it doesn't implement
    // the `PartialEq` trait
    // let _this_is_true = (_one_second == _one_second);
    let foot = Inches(12);
    println!("One foot equals {:?} ", foot);

    let meter = Centimeters(100.00);
    let cmp = if foot.to_centimeters() < meter{
        "Smaller"
    }else{
        "Larger"
    };
    println!("One foot is {} than one meter.", cmp);
}
