// rust by example
// Traits
// A trait is a collection of methods defined for
// an unknown type: Self. They can access other methods
// declared in the same trait. 

pub struct Sheep{
    naked: bool,
    name: &'static str,
}

pub trait Animal{
    // Associated function signature;
    // 'Self' refers to the implementation
    fn new(name: &'static str) -> Self;
    
    // Method signature; these will return a string
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide defualt methods definitions

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep{
    fn is_naked(&self) -> bool{
        self.naked
    }

    pub fn shear(&mut self){
        if self.is_naked() {
            println!("{} is already naked", self.name());
        }else{
            self.naked = true;
            println!("{} gets a haircut!", self.name);
        }
    }
}

impl Animal for Sheep{
    fn new(name: &'static str) -> Sheep {
        Sheep{naked: false, name}
    }

    fn name(&self) -> &'static str {
        self.name
    }
    
    fn noise(&self) -> &'static str {  
        if self.naked {
            "Baa!"
        }else{
            "Moo!"
        }
     }
     // Default trait methods can be overridden
     fn talk(&self){
        println!("{} pauses briefly... {}", self.name, self.noise());
     }
}