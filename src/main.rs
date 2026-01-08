//! Frontier Kingdom - A dark card-based expedition RPG
//! 
//! Built with Macroquad for rendering, input, and audio.
//! Game logic is explicitly state-driven; Macroquad remains thin.

use macroquad::prelude::*;

mod game;
mod state;
mod combat;
mod kingdom;
mod missions;
mod ui;
mod data;
mod save;

use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Frontier Kingdom".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        sample_count: 0,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    
    loop {
        clear_background(Color::from_rgba(20, 20, 25, 255));
        
        game.update();
        game.draw();
        
        next_frame().await;
    }
}
