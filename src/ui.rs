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

        show_jobs_ui(ui, &player);
        show_player_stats_ui(ui, &player);

        // --- Call the UI function and store its output ---
        occupation_event = show_occupation_ui(ui);
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

fn show_jobs_ui(ui: &mut egui::Ui, player: &Player) {
    ui.separator();
    ui.label("Jobs:");
    for job in &player.jobs {
        ui.label(format!("{}: {}", job.name, job.experience));
    }
}

fn show_player_stats_ui(ui: &mut egui::Ui, player: &Player) {
    ui.separator();
    ui.label("Player Stats");
    ui.label(format!("Health: {}", player.health));
    ui.label(format!("Mana: {}", player.mana));
    ui.label(format!("Attack Power: {}", player.attack_power));
    ui.label(format!("Defense: {}", player.defense));
    ui.label(format!("Level: {}", player.level));
    ui.label(format!("Gold: {}", player.gold));
    ui.label(format!("Inventory: {}", player.inventory));
    let current_occupation = match player.current_occupation {
        Some(occ) => format!("{:?}", occ),
        None => "Nothing".to_string(),
    };
    ui.label(format!("Current Occupation: {}", current_occupation));
}
