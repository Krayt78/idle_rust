use std::io::{self, Write};
use crate::player::Player;
use crate::player::Occupation;
use eframe::egui;

pub fn update(player: &mut Player, ctx: &egui::Context) -> Option<Occupation> {
    let mut occupation_event: Option<Occupation> = None; // Initialize event variable

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Idle Game");
        ui.separator();

        ui.label(format!(
            "Current Occupation: {}",
            match player.get_occupation() {
                Some(occ) => format!("{:?}", occ),
                None => "Nothing".to_string(),
            }
        ));

        // --- Call the UI function and store its output ---
        occupation_event = show_occupation_ui(ui);

        // --- Display other player stats ---
        // ...
    });

    // Request a repaint for the next frame - needed for continuous updates
    ctx.request_repaint();

    // Return the event captured during UI drawing
    occupation_event
}

fn show_occupation_ui(ui: &mut egui::Ui) -> Option<Occupation> {
    let mut chosen_occupation = None; // Initialize as None

    ui.separator();
    ui.label("Choose Occupation:");

    // Check buttons and store the choice if clicked
    if ui.button("Mining").clicked() {
        chosen_occupation = Some(Occupation::Miner);
    }
    if ui.button("Woodcutting").clicked() {
        chosen_occupation = Some(Occupation::Woodcutter);
    }
    if ui.button("Farming").clicked() {
        chosen_occupation = Some(Occupation::Farmer);
    }
    // Add more buttons for other occupations...

    chosen_occupation // Return the result (None if no button clicked)
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