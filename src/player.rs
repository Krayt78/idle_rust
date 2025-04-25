use crate::item::Item;

pub enum Occupation {
    None,
    Woodcutter,
    Miner,
    Farmer,
}

pub struct Player {
    pub health: u8,
    pub mana: u8,
    pub attack_power: u8,
    pub defense: u8,
    pub level: u8,
    pub experience: u8,
    pub gold: u8,
    pub inventory: Vec<Item>,
    pub occupation: Occupation,
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
            inventory: Vec::new(),
            occupation: Occupation::None,
        }
    }

    pub fn change_occupation(&mut self, occupation: Occupation) {
        self.occupation = occupation;
    }
}
