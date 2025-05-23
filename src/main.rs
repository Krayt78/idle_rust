mod activity;
mod constants;
mod game_state;
mod inventory;
mod item;
mod job;
mod player;
mod save;
mod ui;
mod quest;
mod utils;

use crate::activity::Activity;
use crate::activity::ActivityName;
use crate::game_state::GameState;
use crate::game_state::QuestState;
use crate::item::Item;
use crate::job::JobName;
use crate::ui::ButtonClicked;
use eframe::egui;
use player::Player;
use crate::utils::load_item_database;
use crate::utils::ItemDatabase;
use crate::utils::load_quest_database;
use crate::utils::QuestDatabase;
use crate::quest::Quest;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]), // Initial window size
        ..Default::default()
    };

    let save = save::load("save.json");
    let mut game_state = GameState::new();
    let mut player = Player::new();
    let mut quests = vec![];
    let mut time_elapsed = 0;

    if save.is_some() {
        game_state = save.clone().unwrap().0;
        player = save.clone().unwrap().1;
        quests = save.clone().unwrap().2;
        let timestamp = save.clone().unwrap().3;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if current_time > timestamp {
            time_elapsed = current_time - timestamp;
        } else {
            println!("What the heck, time went backwards?");
            time_elapsed = 0;
        }

        println!("Time elapsed: {} seconds", time_elapsed);
    }

    // Run the eframe application
    eframe::run_native(
        "Idle Game", // Window title
        options,
        Box::new(move |_cc| Box::new(MyApp::new(player, game_state, quests, time_elapsed))), // Create and run our app
    )
}

// Struct to hold application state
struct MyApp {
    player: Player,
    game_state: GameState,
    quests: Vec<Quest>,
    quest_database: QuestDatabase,
    item_database: ItemDatabase,
}

impl MyApp {
    fn new(mut player: Player, game_state: GameState, mut quests: Vec<Quest>, time_elapsed: u64) -> Self {
        //get the player's current activity and update it based on the time elapsed
        player.update_from_time_elapsed(time_elapsed);

        let item_database = match load_item_database() {
            Ok(item_database) => item_database,
            Err(e) => {
                println!("Error loading item database: {}", e);
                panic!("Failed to load item database");
            }
        };

        let quest_database = match load_quest_database() {
            Ok(quest_database) => quest_database,
            Err(e) => {
                println!("Error loading quest database: {}", e);
                panic!("Failed to load quest database");
            }
        };

        //if the game state has no quests, that means that its a new save
        //so we need to load the quests from the quest database
        if quests.len() == 0 {
            quests = quest_database.values().map(|quest_data| Quest::new(quest_data.id, false)).collect();;
            if quests.len() != quest_database.len() {
                panic!("Quest database and quests vector have different lengths");
            }
        }

        Self { player, game_state, item_database, quest_database, quests }
    }
}

// Implement the eframe::App trait for our struct
impl eframe::App for MyApp {
    // This 'update' function is called on every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let delta_time = ctx.input(|i| i.stable_dt);

        // --- Game Logic using delta_time would go here ---
        // e.g., self.player.passive_update(delta_time);
        self.player.update(delta_time).unwrap();

        // --- Draw UI and get events ---
        // Call ui::update and capture the returned event
        let ui_event = ui::update(&mut self.player, ctx, &self.game_state, &self.quests, &self.item_database, &self.quest_database);

        // --- Handle events returned from UI ---
        if let Some(button_clicked) = ui_event {
            match button_clicked {
                ButtonClicked::Activity => {
                    self.game_state = GameState::Activity;
                }
                ButtonClicked::Crafting => {
                    self.game_state = GameState::Crafting;
                }
                ButtonClicked::Inventory => {
                    self.game_state = GameState::Inventory;
                }
                ButtonClicked::Mining => {
                    self.player.set_activity(Activity::new(
                        ActivityName::Mining,
                        "Mining".to_string(),
                        10.0,
                        vec![(JobName::Miner, 100)],
                        vec![Item::new(2, 1)],
                    ));
                }
                ButtonClicked::Woodcutting => {
                    self.player.set_activity(Activity::new(
                        ActivityName::Woodcutting,
                        "Woodcutting".to_string(),
                        10.0,
                        vec![(JobName::Woodcutter, 100)],
                        vec![Item::new(1, 1)],
                    ));
                }
                ButtonClicked::Farming => {
                    self.player.set_activity(Activity::new(
                        ActivityName::Farming,
                        "Farming".to_string(),
                        10.0,
                        vec![(JobName::Farmer, 100)],
                        vec![Item::new(3, 1)],
                    ));
                }
                ButtonClicked::Quest => {
                    self.game_state = GameState::Quest(QuestState::Available);
                }
                ButtonClicked::AvailableQuests => {
                    self.game_state = GameState::Quest(QuestState::Available);
                }
                ButtonClicked::CompletedQuests => {
                    self.game_state = GameState::Quest(QuestState::Completed);
                }
                ButtonClicked::QuestCompleteClicked(quest_id) => {
                    match self.quests.iter_mut().find(|quest| quest.id == quest_id) {
                        Some(quest) => {
                            quest.complete(&mut self.player, &self.quest_database);
                        }
                        None => {
                            println!("Quest not found");
                            return;
                        }
                    }
                }
                _ => {}
            }
        }

        // Add handling for other potential events from the UI here later...
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Exiting application. saving...");
        save::save(&self.game_state, &self.player, &self.quests, "save.json");
        println!("Save finished.");
    }
}
