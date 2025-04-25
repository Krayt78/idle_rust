use std::io::{self, Write};
use crate::player::Player;

pub fn print_welcome_message() {
    println!("Welcome to the game!");
}

pub fn print_game_loop_message() {
    println!("What task should the character perform?");
}

pub fn print_player_input() {
    println!("What task should the character perform?");
}

pub fn update_screen(player: &Player) {
    //clear the console
    println!("--------------------------------");
    ShowPlayerStats(player);
    println!("--------------------------------");
}

fn ShowPlayerStats(player: &Player) {
    println!("Player Stats");
    println!("Health: {}", player.health);
    println!("Mana: {}", player.mana);
    println!("Attack Power: {}", player.attack_power);
    println!("Defense: {}", player.defense);
    println!("Level: {}", player.level);
    println!("Experience: {}", player.experience);
    println!("Gold: {}", player.gold);
    println!("Inventory: {}", player.inventory);
    println!("Occupation: {}", player.occupation);
}