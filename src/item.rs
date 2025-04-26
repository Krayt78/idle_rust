use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub amount: u128,
}

impl Item {
    pub fn new(name: String, description: String, amount: u128) -> Self {
        Self {
            name,
            description,
            amount,
        }
    }

    pub fn add_amount(&mut self, amount: u128) {
        self.amount += amount;
    }

    pub fn remove_amount(&mut self, amount: u128) {
        self.amount -= amount;
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}
