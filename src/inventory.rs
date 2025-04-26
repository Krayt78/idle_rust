use crate::item::Item;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Inventory {
    pub gold: u128,
    pub items: HashMap<u128, Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            gold: 0,
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: &Item) {
        if let Some(i) = self.items.get_mut(&item.id) {
            i.quantity += item.quantity;
        } else {
            self.items.insert(item.id, item.clone());
        }
    }

    pub fn remove_item(&mut self, item: Item) -> Result<(), String> {
        if let Some(i) = self.items.get_mut(&item.id) {
            if i.quantity > item.quantity {
                i.quantity -= item.quantity;
                Ok(())
            } else if i.quantity == item.quantity {
                self.items.remove(&item.id);
                Ok(())
            } else {
                Err(format!("Item {} has only {} left", item.id, i.quantity))
            }
        } else {
            Err(format!("Item {} not found in inventory", item.id))
        }
    }

    pub fn get_item(&self, item: Item) -> Option<&Item> {
        self.items.get(&item.id)
    }

    pub fn get_item_quantity(&self, item: Item) -> u128 {
        self.get_item(item).map_or(0, |i| i.quantity)
    }

    pub fn add_gold(&mut self, amount: u128) {
        self.gold += amount;
    }

    pub fn remove_gold(&mut self, amount: u128) -> Result<(), String> {
        if self.gold >= amount {
            self.gold -= amount;
            Ok(())
        } else {
            Err(format!("Not enough gold in inventory"))
        }
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_, item) in &self.items {
            write!(f, "{}: {}", item.id, item.quantity)?;
        }
        Ok(())
    }
}
