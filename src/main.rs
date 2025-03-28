

use optiontest::{test_option_chartype, test_option_type, CharacterType};

mod optiontest;

fn main() -> Result<(), String> {
    let n = test_option_type();
    assert_eq!(n.is_some(), true);
    println!("{0}", n.unwrap());

    let m: Option<CharacterType> = test_option_chartype();
    assert_eq!(m.is_some(), true);
    println!("{0}", m.unwrap().to_string());

    Ok(())
}
