use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum GameState {
    Activity,
    Crafting,
    Inventory,
}

impl GameState {
    pub fn new() -> Self {
        Self::Activity
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Activity => write!(f, "Activity"),
            Self::Crafting => write!(f, "Crafting"),
            Self::Inventory => write!(f, "Inventory"),
        }
    }
}
