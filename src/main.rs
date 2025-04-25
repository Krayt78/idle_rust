mod item; // Keep your item module
mod player; // Keep your player module
mod ui; // Keep your ui module
mod inventory; // Keep your inventory module
mod game_loop; // Keep your game_loop module

use eframe::egui; // Import egui for UI elements
use player::{Player, Occupation}; // Import your Player and Occupation

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]), // Initial window size
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // This 'update' function is called on every frame (like your old game loop)

        // --- Calculate Delta Time (Example) ---
        // let delta_time = ctx.input(|i| i.stable_dt); // egui provides delta time
        // You would use delta_time here to update game logic passively

        // --- Define the UI ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Idle Game");
            ui.separator();

            ui.label(format!(
                "Current Occupation: {}",
                // Display the occupation or "None"
                match self.player.get_occupation() { // Assuming you add a get_occupation method
                    Some(occ) => format!("{:?}", occ),
                    None => "Nothing".to_string(),
                }
            ));

            ui.separator();
            ui.label("Choose Occupation:");

            // Add buttons to change occupation
            if ui.button("Mining").clicked() {
                self.player.set_occupation(Occupation::Miner);
            }
            if ui.button("Woodcutting").clicked() {
                self.player.set_occupation(Occupation::Woodcutter);
            }
            if ui.button("Farming").clicked() {
                self.player.set_occupation(Occupation::Farmer);
            }
            // Add more buttons for other occupations

            // --- Display other player stats ---
            // ui.label(format!("Experience: {}", self.player.experience));
            // ui.label(format!("Inventory: {}", ItemListDisplay(&self.player.inventory))); // Using your ItemListDisplay
        });

        // Request a repaint for the next frame - needed for continuous updates
        ctx.request_repaint();
    }
}

// --- You might need to update player.rs slightly ---
// Add a method like this to player.rs:
/*
impl Player {
    // ... existing methods ...

    pub fn get_occupation(&self) -> Option<Occupation> {
        self.current_occupation
    }
}
*/
