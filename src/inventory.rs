use crate::item::Item;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Inventory {
    pub items: HashMap<String, Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: &Item) {
        if let Some(i) = self.items.get_mut(&item.name) {
            i.amount += item.amount;
        } else {
            self.items.insert(item.name.clone(), item.clone());
        }
    }

    pub fn remove_item(&mut self, item: Item) -> Result<(), String> {
        if let Some(i) = self.items.get_mut(&item.name) {
            if i.amount > item.amount {
                i.amount -= item.amount;
                Ok(())
            } else if i.amount == item.amount {
                self.items.remove(&item.name);
                Ok(())
            } else {
                Err(format!("Item {} has only {} left", item.name, i.amount))
            }
        } else {
            Err(format!("Item {} not found in inventory", item.name))
        }
    }

    pub fn get_item(&self, item: Item) -> Option<&Item> {
        self.items.get(&item.name)
    }

    pub fn get_item_amount(&self, item: Item) -> u128 {
        self.get_item(item).map_or(0, |i| i.amount)
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, item) in &self.items {
            write!(f, "{}: {}", name, item.amount)?;
        }
        Ok(())
    }
}
