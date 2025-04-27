use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QuestState {
    Available,
    Completed,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum GameState {
    Activity,
    Crafting,
    Inventory,
    Quest(QuestState),
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
            Self::Quest(_) => write!(f, "Quest"),
        }
    }
}
