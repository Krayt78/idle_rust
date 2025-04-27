use crate::game_state::GameState;
use crate::player::Player;
use eframe::egui;
use crate::utils::ItemDatabase;
use crate::utils::QuestDatabase;
use crate::quest::Quest;
use crate::quest::QuestData;
pub enum ButtonClicked {
    Activity,
    Crafting,
    Inventory,
    Stats,
    Jobs,
    Mining,
    Woodcutting,
    Farming,
    Quest,
}

pub fn update(
    player: &mut Player,
    ctx: &egui::Context,
    game_state: &GameState,
    quests: &Vec<Quest>,
    item_database: &ItemDatabase,
    quest_database: &QuestDatabase,
) -> Option<ButtonClicked> {
    let mut button_clicked: Option<ButtonClicked> = None; // Initialize event variable

    egui::CentralPanel::default().show(ctx, |ui| {
        button_clicked = show_header_ui(ui, game_state);

        if button_clicked.is_none() {
            match game_state {
                GameState::Activity => {
                    button_clicked = show_activity_ui(ui, player, item_database);
                }
                GameState::Crafting => {
                    button_clicked = show_crafting_ui(ui, player, item_database);
                }
                GameState::Inventory => {
                    button_clicked = show_inventory_ui(ui, player, item_database);
                }
                GameState::Quest => {
                    button_clicked = show_quest_ui(ui, player, quests, quest_database, item_database);
                }
                _ => {}
            }
        }
    });

    // Request a repaint for the next frame - needed for continuous updates
    ctx.request_repaint();

    // Return the event captured during UI drawing
    button_clicked
}

fn show_header_ui(ui: &mut egui::Ui, game_state: &GameState) -> Option<ButtonClicked> {
    let mut button_clicked = None;

    //the button for the gamestate we are in should be disabled
    ui.horizontal(|ui| {
        ui.heading(format!("{}", game_state));
        ui.separator();

        match game_state {
            GameState::Activity => {
                ui.add_enabled(false, egui::Button::new("Activity"));
                if ui.button("Crafting").clicked() {
                    button_clicked = Some(ButtonClicked::Crafting);
                }
                if ui.button("Inventory").clicked() {
                    button_clicked = Some(ButtonClicked::Inventory);
                }
                if ui.button("Quest").clicked() {
                    button_clicked = Some(ButtonClicked::Quest);
                }
            }
            GameState::Crafting => {
                if ui.button("Activity").clicked() {
                    button_clicked = Some(ButtonClicked::Activity);
                }
                ui.add_enabled(false, egui::Button::new("Crafting"));
                if ui.button("Inventory").clicked() {
                    button_clicked = Some(ButtonClicked::Inventory);
                }
                if ui.button("Quest").clicked() {
                    button_clicked = Some(ButtonClicked::Quest);
                }
            }
            GameState::Inventory => {
                if ui.button("Activity").clicked() {
                    button_clicked = Some(ButtonClicked::Activity);
                }
                if ui.button("Crafting").clicked() {
                    button_clicked = Some(ButtonClicked::Crafting);
                }
                ui.add_enabled(false, egui::Button::new("Inventory"));
                if ui.button("Quest").clicked() {
                    button_clicked = Some(ButtonClicked::Quest);
                }
            }
            GameState::Quest => {
                if ui.button("Activity").clicked() {
                    button_clicked = Some(ButtonClicked::Activity);
                }
                if ui.button("Crafting").clicked() {
                    button_clicked = Some(ButtonClicked::Crafting);
                }
                if ui.button("Inventory").clicked() {
                    button_clicked = Some(ButtonClicked::Inventory);
                }
                ui.add_enabled(false, egui::Button::new("Quest"));
            }
        }
    });
    ui.separator();
    button_clicked
}

fn show_activity_ui(
    ui: &mut egui::Ui,
    player: &mut Player,
    item_database: &ItemDatabase,
) -> Option<ButtonClicked> {
    let mut button_clicked = None; // Initialize as None

    let current_activity = player.get_activity();

    ui.label(format!(
        "Current Activity: {}",
        match current_activity {
            Some(act) => {
                format!("{:?}", act.name)
            }
            None => {
                "Nothing".to_string()
            }
        }
    ));

    if let Some(act) = current_activity {
        ui.add(egui::ProgressBar::new(act.timer / act.duration));
    } else {
        ui.add(egui::ProgressBar::new(0.0));
    }

    show_jobs_ui(ui, &player);
    show_player_stats_ui(ui, &player, item_database);

    ui.separator();
    ui.label("Choose Activity:");

    // Check buttons and store the choice if clicked
    if ui.button("Mining").clicked() {
        button_clicked = Some(ButtonClicked::Mining);
    }
    if ui.button("Woodcutting").clicked() {
        button_clicked = Some(ButtonClicked::Woodcutting);
    }
    if ui.button("Farming").clicked() {
        button_clicked = Some(ButtonClicked::Farming);
    }
    // Add more buttons for other occupations...

    button_clicked // Return the result (None if no button clicked)
}

fn show_crafting_ui(
    ui: &mut egui::Ui,
    player: &mut Player,
    item_database: &ItemDatabase,
) -> Option<ButtonClicked> {
    let mut button_clicked = None;

    button_clicked
}

fn show_inventory_ui(
    ui: &mut egui::Ui,
    player: &mut Player,
    item_database: &ItemDatabase,
) -> Option<ButtonClicked> {
    let mut button_clicked = None;

    button_clicked
}

fn show_quest_ui(
    ui: &mut egui::Ui,
    player: &mut Player,
    quests: &Vec<Quest>,
    quest_database: &QuestDatabase,
    item_database: &ItemDatabase,
) -> Option<ButtonClicked> {
    let mut button_clicked = None;

    for quest in quests {
        let quest_data = quest_database.get(&quest.id).unwrap();
        quest_ui_component(ui, quest, quest_data);
    }

    button_clicked
}

fn quest_ui_component(ui: &mut egui::Ui, quest: &Quest, quest_data: &QuestData) {
   //todo: implement quest ui component
}

fn show_jobs_ui(ui: &mut egui::Ui, player: &Player) {
    ui.separator();
    ui.label("Jobs:");
    for job in &player.jobs {
        ui.label(format!(
            "{}: level: {} | xp: {}",
            job.name, job.level, job.experience
        ));
    }
}

fn show_player_stats_ui(
    ui: &mut egui::Ui,
    player: &Player,
    item_database: &ItemDatabase,
) {
    ui.separator();
    ui.label("Player Stats");
    ui.label(format!("Health: {}", player.health));
    ui.label(format!("Mana: {}", player.mana));
    ui.label(format!("Attack Power: {}", player.attack_power));
    ui.label(format!("Defense: {}", player.defense));
    ui.label(format!("Level: {}", player.level));
    ui.label(format!("Gold: {}", player.inventory.gold));
    // Display inventory with names
    ui.label("Inventory:");
    if player.inventory.items.is_empty() {
        ui.label("  (Empty)");
    } else {
        for (id, item) in &player.inventory.items {
            let item_name = item_database
                .get(id)
                .map_or("Unknown Item", |data| &data.name);
            ui.label(format!("  {}: {}", item_name, item.quantity));
        }
    }
    let current_activity = match &player.current_activity {
        Some(act) => format!("{:?}", act.name),
        None => "Nothing".to_string(),
    };
    ui.label(format!("Current Activity: {}", current_activity));
}
