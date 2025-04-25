use std::io::{self, Write};
use crate::player::Player;
use crate::activity::Activity;
use crate::activity::ActivityName;
use crate::job::JobName;
use eframe::egui;

pub fn update(player: &mut Player, ctx: &egui::Context) -> Option<Activity> {
    let mut activity_event: Option<Activity> = None; // Initialize event variable

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Idle Game");
        ui.separator();

        ui.label(format!(
            "Current Activity: {}",
            match player.get_activity() {
                Some(act) => format!("{:?}", act.name),
                None => "Nothing".to_string(),
            }
        ));

        show_jobs_ui(ui, &player);
        show_player_stats_ui(ui, &player);

        // --- Call the UI function and store its output ---
        activity_event = show_activity_ui(ui);
    });

    // Request a repaint for the next frame - needed for continuous updates
    ctx.request_repaint();

    // Return the event captured during UI drawing
    activity_event
}

fn show_activity_ui(ui: &mut egui::Ui) -> Option<Activity> {
    let mut chosen_activity = None; // Initialize as None

    ui.separator();
    ui.label("Choose Activity:");

    // Check buttons and store the choice if clicked
    if ui.button("Mining").clicked() {
        chosen_activity = Some(Activity::new(ActivityName::Mining, "Mining".to_string(), 10.0, vec![(JobName::Miner, 100)], vec![]));
    }
    if ui.button("Woodcutting").clicked() {
        chosen_activity = Some(Activity::new(ActivityName::Woodcutting, "Woodcutting".to_string(), 10.0, vec![(JobName::Woodcutter, 100)], vec![]));
    }
    if ui.button("Farming").clicked() {
        chosen_activity = Some(Activity::new(ActivityName::Farming, "Farming".to_string(), 10.0, vec![(JobName::Farmer, 100)], vec![]));
    }
    // Add more buttons for other occupations...

    chosen_activity // Return the result (None if no button clicked)
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
    let current_activity = match &player.current_activity {
        Some(act) => format!("{:?}", act.name),
        None => "Nothing".to_string(),
    };
    ui.label(format!("Current Activity: {}", current_activity));
}
