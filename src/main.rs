pub mod helpers;

use crate::helpers::namehelpers::get_full_name;
fn main() {

    let full_name = get_full_name("Pedro", "Martinez");
    println!("Full name: {}", full_name);

}

