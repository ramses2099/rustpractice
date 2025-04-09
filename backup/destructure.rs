#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    real_name: String,
    heigth: u8,
    happy: bool,
}
fn main() -> Result<(), String> {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarences".to_string(),
        heigth: 170,
        happy: false,
    };

    println!("{:?}", papa_doc);
    // Destructure struct
    let Person { name, real_name, heigth, happy } = papa_doc;
    println!("name {}, real name {}", name, real_name);

    Ok(())
}
