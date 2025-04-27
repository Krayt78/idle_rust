use crate::item::ItemData;
use std::collections::HashMap;
use crate::constants::ITEM_DATABASE_PATH;
use crate::constants::QUEST_DATABASE_PATH;
use crate::quest::QuestData;
use std::fs::File;
use std::io::BufReader;

pub type ItemDatabase = HashMap<u128, ItemData>;
pub fn load_item_database() -> Result<ItemDatabase, Box<dyn std::error::Error>> {
    let file = File::open(ITEM_DATABASE_PATH)?;
    let reader = BufReader::new(file);

    // 1. Deserialize the JSON array into a Vec first
    let items_vec: Vec<ItemData> = serde_json::from_reader(reader)?;

    // 2. Convert the Vec into a HashMap
    let mut item_db = ItemDatabase::new();
    for item_def in items_vec {
        item_db.insert(item_def.id, item_def); // Use item's ID as the key
    }

    Ok(item_db)
}

pub type QuestDatabase = HashMap<u128, QuestData>;
pub fn load_quest_database() -> Result<QuestDatabase, Box<dyn std::error::Error>> {
    let file = File::open(QUEST_DATABASE_PATH)?;
    let reader = BufReader::new(file);

    let quests_vec: Vec<QuestData> = serde_json::from_reader(reader)?;

    let mut quest_db = QuestDatabase::new();
    for quest in quests_vec {
        quest_db.insert(quest.id, quest);
    }

    Ok(quest_db)
}

mod tests {
    use super::*;

    #[test]
    fn test_load_item_database() {
        let item_database = load_item_database().unwrap();
        //just check that its not empty
        assert!(!item_database.is_empty());
    }

    #[test]
    fn test_load_quest_database() {
        let quest_database = load_quest_database().unwrap();
        //just check that its not empty
        assert!(!quest_database.is_empty());
    }
}
