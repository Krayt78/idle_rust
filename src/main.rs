mod activity;
mod constants;
mod game_state;
mod inventory;
mod item;
mod job;
mod player;
mod save;
mod ui;

use crate::activity::Activity;
use crate::activity::ActivityName;
use crate::game_state::GameState;
use crate::item::Item;
use crate::job::JobName;
use crate::ui::ButtonClicked;
use eframe::egui;
use player::Player;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]), // Initial window size
        ..Default::default()
    };

    let save = save::load("save.json");
    let mut game_state = GameState::new();
    let mut player = Player::new();
    let mut time_elapsed = 0;

    if save.is_some() {
        game_state = save.clone().unwrap().0;
        player = save.clone().unwrap().1;
        let timestamp = save.clone().unwrap().2;
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
        Box::new(move |_cc| Box::new(MyApp::new(player, game_state, time_elapsed))), // Create and run our app
    )
}

// Struct to hold application state
struct MyApp {
    player: Player,
    game_state: GameState,
    // You might add other state here later, like last update time for delta_t
}

impl MyApp {
    fn new(mut player: Player, game_state: GameState, time_elapsed: u64) -> Self {
        //get the player's current activity and update it based on the time elapsed
        player.update_from_time_elapsed(time_elapsed);

        Self { player, game_state }
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
        let ui_event = ui::update(&mut self.player, ctx, &self.game_state);

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
                        vec![Item::new("Stone".to_string(), "Stone".to_string(), 1)],
                    ));
                }
                ButtonClicked::Woodcutting => {
                    self.player.set_activity(Activity::new(
                        ActivityName::Woodcutting,
                        "Woodcutting".to_string(),
                        10.0,
                        vec![(JobName::Woodcutter, 100)],
                        vec![Item::new("Log".to_string(), "Log".to_string(), 1)],
                    ));
                }
                ButtonClicked::Farming => {
                    self.player.set_activity(Activity::new(
                        ActivityName::Farming,
                        "Farming".to_string(),
                        10.0,
                        vec![(JobName::Farmer, 100)],
                        vec![Item::new("Potato".to_string(), "Potato".to_string(), 1)],
                    ));
                }
                _ => {}
            }
        }

        // Add handling for other potential events from the UI here later...
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Exiting application. saving...");
        save::save(&self.game_state, &self.player, "save.json");
        println!("Save finished.");
    }
}
