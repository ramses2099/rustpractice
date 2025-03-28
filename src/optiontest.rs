#![allow(dead_code)]
#![allow(unused_variables)]

// #region function test
pub fn test_option_type() -> Option<u8> {
    Some(5)
}

pub fn test_option_chartype() -> Option<CharacterType> {
    let m = Some(CharacterType::Warrior); 
    m   
}

pub enum CharacterType{
    Archer = 1,
    Warrior,
    Mage
}

impl ToString for CharacterType{
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer".to_string(),
            CharacterType::Warrior => "Warrior".to_string(),
            CharacterType::Mage => "Mage".to_string(),
        }        
    }
}

// #endregion
