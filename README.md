# Idle Rust Game

This is a simple idle game built using Rust and the [`egui`](https://github.com/emilk/egui) immediate mode GUI library via the [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe) framework.

The game allows the player character to perform various activities like Mining, Woodcutting, and Farming to passively gain experience and items over time, even when the game window is closed (progress is calculated based on elapsed time when restarting).

## Features

*   Choose different activities (Mining, Woodcutting, Farming).
*   Passively gain job experience and items based on the selected activity.
*   Simple GUI built with `egui`.
*   Game state (player progress, current activity, inventory) is saved to `save.json` when the application is closed.
*   Calculates offline progress based on the time elapsed since the last session.

## Running the Game

1.  **Ensure you have Rust installed:** If not, follow the instructions at [rustup.rs](https://rustup.rs/).
2.  **Clone the repository (if you haven't already):**
    ```bash
    git clone https://github.com/Krayt78/idle_rust.git
    cd idle_rust
    ```
3.  **Build and run the game:**
    ```bash
    cargo run
    ```
    This command will compile the project and launch the game window.

## Dependencies

*   Rust programming language
*   `eframe` / `egui`: For the graphical user interface.
*   `serde`: For serializing and deserializing game state to/from the save file.

## Saving

The game automatically saves the current state (player stats, inventory, jobs, current activity, game view state) to a file named `save.json` in the project's root directory when you close the application window. When you restart the game, it will attempt to load this file and calculate any progress made while the game was closed. 