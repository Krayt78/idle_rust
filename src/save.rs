//save the game state to a file

use crate::game_state::GameState;
use crate::player::Player;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

pub fn save(game_state: &GameState, player: &Player, save_name: &str) {
    let file = File::create(save_name).unwrap();
    let mut writer = BufWriter::new(file);

    //save the game state, player to the file
    let save = Save {
        game_state: game_state.clone(),
        player: player.clone(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    serde_json::to_writer(&mut writer, &save).unwrap();
}

pub fn load(save_name: &str) -> Option<(GameState, Player, u64)> {
    match File::open(save_name) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let save: Save = match serde_json::from_reader(&mut reader) {
                Ok(save) => save,
                Err(_) => return None,
            };
            Some((save.game_state, save.player, save.timestamp))
        }
        Err(_) => None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct Save {
    pub game_state: GameState,
    pub player: Player,
    pub timestamp: u64,
}

mod tests {
    use super::*;

    #[test]
    fn test_save() {
        let game_state = GameState::new();
        let player = Player::new();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let save_name = "test_save.json";

        save(&game_state, &player, save_name);

        let file = File::open(save_name).unwrap();
        let mut reader = BufReader::new(file);

        let save: Save = serde_json::from_reader(&mut reader).unwrap();

        assert_eq!(game_state, save.game_state);
        assert_eq!(player, save.player);
        assert_eq!(timestamp, save.timestamp);
    }

    #[test]
    fn test_save_and_load() {
        let game_state = GameState::new();
        let player = Player::new();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let save_name = "test_save.json";

        save(&game_state, &player, save_name);

        let (loaded_game_state, loaded_player, loaded_timestamp) = load(save_name).unwrap();

        assert_eq!(game_state, loaded_game_state);
        assert_eq!(player, loaded_player);
        assert_eq!(timestamp, loaded_timestamp);
    }
}
