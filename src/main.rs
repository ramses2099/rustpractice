mod mod01;
use mod01::Sheep;
use mod01::Animal;
fn main() {
   let mut dolly: Sheep = Animal::new("Dolly");
   dolly.talk();
   dolly.shear();
   dolly.talk();
   
}
