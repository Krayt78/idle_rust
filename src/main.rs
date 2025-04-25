mod activity; // Keep your activity module
mod game_state;
mod inventory; // Keep your inventory module
mod item; // Keep your item module
mod job;
mod player; // Keep your player module
mod ui; // Keep your ui module // Keep your job module

use crate::activity::Activity;
use crate::activity::ActivityName;
use crate::game_state::GameState;
use crate::inventory::Inventory; // Assuming you might want to display inventory later
use crate::item::Item;
use crate::job::JobName;
use crate::ui::ButtonClicked;
use eframe::egui; // Import egui for UI elements
use player::Player; // Import your Player // Assuming you might want to display items later

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]), // Initial window size
        ..Default::default()
    };

    // Create the initial player state
    let initial_player = Player::new(); // You might want to set an initial occupation later

    // Run the eframe application
    eframe::run_native(
        "Idle Game", // Window title
        options,
        Box::new(|_cc| Box::new(MyApp::new(initial_player))), // Create and run our app
    )
}

// Struct to hold application state
struct MyApp {
    player: Player,
    game_state: GameState,
    // You might add other state here later, like last update time for delta_t
}

impl MyApp {
    fn new(player: Player) -> Self {
        Self {
            player,
            game_state: GameState::new(),
        }
    }
}

// Implement the eframe::App trait for our struct
impl eframe::App for MyApp {
    // This 'update' function is called on every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let delta_time = ctx.input(|i| i.stable_dt);

        // --- Game Logic using delta_time would go here ---
        // e.g., self.player.passive_update(delta_time);
        self.player.update(delta_time);

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
}
