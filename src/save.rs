//save the game state to a file

use crate::game_state::GameState;
use crate::player::Player;
use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;
use serde_json;

pub fn save(game_state: &GameState, player: &Player) {
    let file = File::create("save.json").unwrap();
    let mut writer = BufWriter::new(file);

    //save the game state, player to the file
    serde_json::to_writer(&mut writer, game_state).unwrap();
    serde_json::to_writer(&mut writer, player).unwrap();
}

pub fn load(game_state: &mut GameState, player: &mut Player) {
    let file = File::open("save.json").unwrap();
    let reader = BufReader::new(file);
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

        let loaded_game_state: GameState = serde_json::from_reader(&mut reader).unwrap();
        let loaded_player: Player = serde_json::from_reader(&mut reader).unwrap();

        assert_eq!(game_state, loaded_game_state);
        assert_eq!(player, loaded_player);
    }
}