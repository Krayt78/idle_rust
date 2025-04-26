use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Item {
    pub id: u128,
    pub quantity: u128,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ItemData {
    pub id: u128,
    pub name: String,
    pub description: String,
}

impl Item {
    pub fn new(id: u128, quantity: u128) -> Self {
        Self {
            id,
            quantity,
        }
    }

    pub fn add_quantity(&mut self, quantity: u128) {
        self.quantity += quantity;
    }

    pub fn remove_quantity(&mut self, quantity: u128) {
        self.quantity -= quantity;
    }
}

impl fmt::Display for ItemData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}
