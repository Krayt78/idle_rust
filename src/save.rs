//save the game state to a file

use crate::game_state::GameState;
use crate::player::Player;
use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;
use serde_json;
use serde::{Serialize, Deserialize};

pub fn save(game_state: &GameState, player: &Player) {
    let file = File::create("save.json").unwrap();
    let mut writer = BufWriter::new(file);

    //save the game state, player to the file
    let save = Save {
        game_state: game_state.clone(),
        player: player.clone(),
    };

    serde_json::to_writer(&mut writer, &save).unwrap();
}

pub fn load() -> Option<(GameState, Player)> {
    let file = File::open("save.json").unwrap();
    let mut reader = BufReader::new(file);

    let save: Save = match serde_json::from_reader(&mut reader) {
        Ok(save) => save,
        Err(_) => return None,
    };

    Some((save.game_state, save.player))
}

#[derive(Serialize, Deserialize)]
pub struct Save {
    pub game_state: GameState,
    pub player: Player,
}

mod tests {
    use super::*;

    #[test]
    fn test_save() {
        let game_state = GameState::new();
        let player = Player::new();

        save(&game_state, &player);

        let file = File::open("save.json").unwrap();
        let mut reader = BufReader::new(file);

        let save: Save = serde_json::from_reader(&mut reader).unwrap();

        assert_eq!(game_state, save.game_state);
        assert_eq!(player, save.player);
    }

    #[test]
    fn test_save_and_load() {
        let game_state = GameState::new();
        let player = Player::new();

        save(&game_state, &player);

        let (loaded_game_state, loaded_player) = load().unwrap();

        assert_eq!(game_state, loaded_game_state);
        assert_eq!(player, loaded_player);
    }
}