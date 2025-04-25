use crate::item::Item;
use std::fmt;
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Item) {
        if let Some(i) = self.items.iter_mut().find(|i| i.name == item.name) {
            i.amount += item.amount;
        } else {
            self.items.push(item);
        }
    }

    pub fn remove_item(&mut self, item: Item) -> Result<(), String> {
        if let Some(i) = self.items.iter_mut().find(|i| i.name == item.name) {
            if i.amount > item.amount {
                i.amount -= item.amount;
                Ok(())
            } else if i.amount == item.amount {
                self.items.remove(self.items.iter().position(|i| i.name == item.name).unwrap());
                Ok(())
            } else {
                Err(format!("Item {} has only {} left", item.name, i.amount))
            }
        } else {
            Err(format!("Item {} not found in inventory", item.name))
        }
    }

    pub fn get_item(&self, item: Item) -> Option<&Item> {
        self.items.iter().find(|i| i.name == item.name)
    }

    pub fn get_item_amount(&self, item: Item) -> u128 {
        self.get_item(item).map_or(0, |i| i.amount)
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.items {
            write!(f, "{}: {}", item.name, item.amount)?;
        }
        Ok(())
    }
}

