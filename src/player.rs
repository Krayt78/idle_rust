use crate::inventory::Inventory;
use crate::item::Item;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Occupation {
    Woodcutter,
    Miner,
    Farmer,
}

pub struct Job {
    pub name: String,
    pub description: String,
    pub experience: u128,
}

impl Job {
    pub fn new(name: String, description: String, experience: u128) -> Self {
        Self { name, description, experience }
    }
}

impl fmt::Display for Occupation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
    pub jobs: Vec<Job>,
    pub gold: u8,
    pub inventory: Inventory,
    pub current_occupation: Option<Occupation>,
}

impl Player {
    pub fn new() -> Self {
        let jobs = vec![
            Job::new("Woodcutter".to_string(), "Cut down trees".to_string(), 0),
            Job::new("Miner".to_string(), "Mine rocks".to_string(), 0),
            Job::new("Farmer".to_string(), "Grow crops".to_string(), 0),
        ];
        Self {
            health: 100,
            mana: 100,
            attack_power: 1,
            defense: 1,
            level: 1,
            jobs: jobs,
            gold: 0,
            inventory: Inventory::new(),
            current_occupation: None,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Update player stats based on current occupation
        match self.current_occupation {
            Some(Occupation::Woodcutter) => {
                self.inventory.add_item(Item::new("Log".to_string(), "A Log".to_string(), 1));
                self.jobs[0].experience += 1;
            }
            Some(Occupation::Miner) => {
                self.inventory.add_item(Item::new("Stone".to_string(), "A Stone".to_string(), 1));
                self.jobs[1].experience += 1;
            }
            Some(Occupation::Farmer) => {
                self.inventory.add_item(Item::new("Wheat".to_string(), "Some Wheat".to_string(), 1));
                self.jobs[2].experience += 1;
            }
            _ => {}
        }
    }   

    pub fn change_occupation(&mut self, occupation: Occupation) {
        self.current_occupation = Some(occupation);
        println!("Player is now {:?}", occupation);
    }

    pub fn set_occupation(&mut self, occupation: Occupation) {
        self.current_occupation = Some(occupation);
        println!("Player is now {:?}", occupation);
    }

    pub fn get_occupation(&self) -> Option<Occupation> {
        self.current_occupation
    }
}
