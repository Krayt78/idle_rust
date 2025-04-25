use crate::player::Player; // Import the Player struct
use crate::ui::update_screen;
use std::thread::sleep;
use std::time::{Duration, Instant};

// Function to contain the game loop logic
pub fn run_game_loop(mut player: Player) {
    // Pass the player as an argument
    let mut last_frame_time = Instant::now();

    loop {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time);
        last_frame_time = now;

        update_screen(&player);

        sleep(Duration::from_millis(1));
    }
}
