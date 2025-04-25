use crate::inventory::Inventory;
use crate::item::Item;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Occupation {
    None,
    Woodcutter,
    Miner,
    Farmer,
}

impl fmt::Display for Occupation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Occupation::None => write!(f, "None"),
            Occupation::Woodcutter => write!(f, "Woodcutter"),
            Occupation::Miner => write!(f, "Miner"),
            Occupation::Farmer => write!(f, "Farmer"),
        }
    }
}

pub struct Player {
    pub health: u8,
    pub mana: u8,
    pub attack_power: u8,
    pub defense: u8,
    pub level: u8,
    pub experience: u8,
    pub gold: u8,
    pub inventory: Inventory,
    pub occupation: Occupation,
    pub current_occupation: Option<Occupation>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100,
            mana: 100,
            attack_power: 1,
            defense: 1,
            level: 1,
            experience: 0,
            gold: 0,
            inventory: Inventory::new(),
            occupation: Occupation::None,
            current_occupation: None,
        }
    }

    pub fn change_occupation(&mut self, occupation: Occupation) {
        self.occupation = occupation;
    }

    pub fn set_occupation(&mut self, occupation: Occupation) {
        self.current_occupation = Some(occupation);
        println!("Player is now {:?}", occupation);
    }

    pub fn get_occupation(&self) -> Option<Occupation> {
        self.current_occupation
    }
}
