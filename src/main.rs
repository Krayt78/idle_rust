mod item; // Keep your item module
mod player; // Keep your player module
mod ui; // Keep your ui module
mod inventory; // Keep your inventory module
mod activity; // Keep your activity module
mod job; // Keep your job module    

use eframe::egui; // Import egui for UI elements
use player::Player; // Import your Player
use crate::inventory::Inventory; // Assuming you might want to display inventory later
use crate::item::Item; // Assuming you might want to display items later

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
    // You might add other state here later, like last update time for delta_t
}

impl MyApp {
    fn new(player: Player) -> Self {
        Self { player }
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
        let ui_event = ui::update(&mut self.player, ctx);

        // --- Handle events returned from UI ---
        if let Some(chosen_activity) = ui_event {
             // Check if the event was an Activity choice
             // In the future, ui::update might return different kinds of events
             // using an enum, so a match might be better here.
            self.player.set_activity(chosen_activity);
        }

        // Add handling for other potential events from the UI here later...
    }
}
