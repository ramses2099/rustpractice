use crate::states::mischievous::MischievousState;
pub struct Cat{
    name: String
}

impl Cat{
    pub fn new(name: String) -> Mischievous<Self>{
        Mischievous::new(Self{name})
    }
}