pub trait Vehicle{
    fn driver(&self);
}
pub struct Truck;

impl Vehicle for Truck {
    fn driver(&self) {
        println!("I'm driving a truck!");
    }
}