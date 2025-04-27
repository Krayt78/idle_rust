use crate::game_state::GameState;
use crate::player::Player;
use eframe::egui;
use crate::utils::ItemDatabase;
use crate::utils::QuestDatabase;
use crate::quest::Quest;
use crate::quest::QuestData;
use crate::game_state::QuestState;
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
    AvailableQuests,
    CompletedQuests,
    QuestCompleteClicked(u128),
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
                GameState::Quest(QuestState::Available) => {
                    button_clicked = show_available_quests_ui(ui, quests, quest_database, item_database, player);
                }
                GameState::Quest(QuestState::Completed) => {
                    button_clicked = show_completed_quests_ui(ui, quests, quest_database, item_database, player);
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
            GameState::Quest(_) => {
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

fn show_quests_ui(
    ui: &mut egui::Ui,
    quests: &Vec<Quest>,
    quest_database: &QuestDatabase,
) -> Option<ButtonClicked> {
    let mut button_clicked = None; // Initialize as None

    // Check buttons and store the choice if clicked
    if ui.button("Available Quests").clicked() {
        button_clicked = Some(ButtonClicked::AvailableQuests);
    }
    if ui.button("Completed Quests").clicked() {
        button_clicked = Some(ButtonClicked::CompletedQuests);
    }

    button_clicked // Return the result (None if no button clicked)
}

// Helper function to format quest goals, showing progress
fn format_goal(goal: &crate::quest::Goal, item_database: &ItemDatabase, player: &Player) -> String {
    match &goal.objective {
        crate::quest::Objective::CollectItem(item_id) => {
            let item_name = item_database.get(item_id).map_or("Unknown Item", |d| d.name.as_str());
            let current = player.inventory.get_item_quantity(*item_id);
            format!("Collect {} {} ({}/{})", goal.required_amount, item_name, current, goal.required_amount)
        }
        crate::quest::Objective::CollectGold() => {
            let current = player.inventory.gold;
            format!("Collect {} Gold ({}/{})", goal.required_amount, current, goal.required_amount)
        }
        crate::quest::Objective::ReachJobLevel(job_name) => {
            // Use u128::from to ensure type compatibility for comparison display
            let current = player.get_job(job_name.clone()).map_or(0, |j| u128::from(j.level));
            format!("Reach Level {} in {:?} ({}/{})", goal.required_amount, job_name, current, goal.required_amount)
        }
        crate::quest::Objective::ReachLevel() => {
             // Use u128::from to ensure type compatibility for comparison display
            let current = u128::from(player.level);
            format!("Reach Player Level {} ({}/{})", goal.required_amount, current, goal.required_amount)
        }
    }
}

fn is_goal_reached(goal: &crate::quest::Goal, player: &Player) -> bool {
    match &goal.objective {
        crate::quest::Objective::CollectItem(item_id) => {
            player.inventory.get_item_quantity(*item_id) >= goal.required_amount
        }
        crate::quest::Objective::CollectGold() => {
            player.inventory.gold >= goal.required_amount
        }
        crate::quest::Objective::ReachJobLevel(job_name) => {
            player.get_job(job_name.clone()).map_or(false, |j| u128::from(j.level) >= goal.required_amount)
        }
        crate::quest::Objective::ReachLevel() => {
            u128::from(player.level) >= goal.required_amount
        }
    }
}

fn format_completed_goal(goal: &crate::quest::Goal, item_database: &ItemDatabase, player: &Player) -> String {
    match &goal.objective {
        crate::quest::Objective::CollectItem(item_id) => {
            let item_name = item_database.get(item_id).map_or("Unknown Item", |d| d.name.as_str());
            format!("Collect {} {}", goal.required_amount, item_name)
        }
        crate::quest::Objective::CollectGold() => {
            format!("Collect {} Gold", goal.required_amount)
        }
        crate::quest::Objective::ReachJobLevel(job_name) => {
            format!("Reach Level {} in {:?}", goal.required_amount, job_name)
        }
        crate::quest::Objective::ReachLevel() => {
            format!("Reach Player Level {}", goal.required_amount)
        }
    }
}
// Helper function to format quest rewards
fn format_reward(reward: &crate::quest::Reward, item_database: &ItemDatabase) -> String {
    let mut parts = Vec::new();
    // Use the tuple format as per the current quest.rs code
    if let Some(experience) = &reward.experience {
        parts.push(format!("{} {:?} XP", experience.amount, experience.job));
    }
    if let Some(items) = &reward.items {
        for item in items {
             let item_name = item_database.get(&item.id).map_or("Unknown Item", |d| d.name.as_str());
             parts.push(format!("{}x {}", item.quantity, item_name));
        }
    }
    if let Some(gold) = &reward.gold {
        parts.push(format!("{} Gold", gold));
    }
    if parts.is_empty() {
        "Nothing".to_string()
    } else {
        parts.join(", ")
    }
}

fn show_available_quests_ui(
    ui: &mut egui::Ui,
    quests: &Vec<Quest>,
    quest_database: &QuestDatabase,
    item_database: &ItemDatabase,
    player: &Player,
) -> Option<ButtonClicked> {
    let mut button_clicked = None;
    let mut quest_to_complete_id: Option<u128> = None;

    ui.separator();
    ui.label("Available Quests");
    ui.separator();
    ui.add_enabled(false, egui::Button::new("Available Quests"));
    if ui.button("Completed Quests").clicked() {
        button_clicked = Some(ButtonClicked::CompletedQuests);
    }
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for quest in quests {
            if !quest.completed {
                let quest_data = quest_database.get(&quest.id).unwrap();
                if quest_ui_component(ui, quest_data, item_database, player, true) {
                    button_clicked = Some(ButtonClicked::QuestCompleteClicked(quest.id));
                }
                ui.separator();
            }
        }
    });

    button_clicked
}

fn show_completed_quests_ui(
    ui: &mut egui::Ui,
    quests: &Vec<Quest>,
    quest_database: &QuestDatabase,
    item_database: &ItemDatabase,
    player: &Player,
) -> Option<ButtonClicked> {
    let mut button_clicked = None;

    ui.separator();
    ui.label("Completed Quests");
    ui.separator();
    if ui.button("Available Quests").clicked() {
        button_clicked = Some(ButtonClicked::AvailableQuests);
    }
    ui.add_enabled(false, egui::Button::new("Completed Quests"));
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for quest in quests {
            if quest.completed {
                let quest_data = quest_database.get(&quest.id).unwrap();
                quest_ui_component(ui, quest_data, item_database, player, false);
                ui.separator();
            }
        }
    });

    button_clicked
}

fn quest_ui_component(
    ui: &mut egui::Ui,
    quest_data: &QuestData,
    item_database: &ItemDatabase,
    player: &Player,
    is_completable: bool,
) -> bool {
    let mut clicked = false;
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width() * 0.9);

        ui.label(egui::RichText::new(&quest_data.name).strong());
        ui.separator();

        ui.label(&quest_data.description);
        ui.add_space(4.0);

        if is_completable {
            ui.label(format!("Goal: {}", format_goal(&quest_data.goal, item_database, player)));
            ui.add_space(4.0);
        } else {
            ui.label(format!("Goal: {}", format_completed_goal(&quest_data.goal, item_database, player)));
            ui.add_space(4.0);
        }

        ui.label(format!("Reward: {}", format_reward(&quest_data.reward, item_database)));
         
        if is_completable {
            if is_goal_reached(&quest_data.goal, player) {
                ui.add_space(8.0);
                ui.horizontal(|ui|{
                    ui.add_space(ui.available_width() * 0.25);
                    if ui.add_sized([ui.available_width() * 0.5, 20.0], egui::Button::new("Attempt Completion")).clicked() {
                        clicked = true;
                    }
                });
            }
        } else {
            ui.add_space(8.0);
            ui.horizontal(|ui|{
                ui.add_space(ui.available_width() * 0.3);
                ui.label(egui::RichText::new("(Quest Completed)").color(egui::Color32::DARK_GREEN));
            });
        }
    });
    ui.add_space(5.0);
    clicked
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
