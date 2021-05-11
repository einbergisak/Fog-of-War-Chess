use std::{
    io::{self},
    sync::RwLock,
};

use game::Game;
use ggez::event::{self};
use ggez::{
    conf,
    graphics::{self, Rect},
    ContextBuilder,
};
use state::Storage;

mod default_board_state;
mod event_handler;
mod game;
mod piece;
mod piece_movement;
mod render_utilities;
mod networking {
    pub mod connection;
    pub mod events;
}
mod menu {
    pub mod menu_state;
    pub mod clickable;
    pub mod menu_utilities;
}

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) count: i32,
    pub(crate) incoming_move: Option<(usize, usize)>,
}

static STATE: Storage<RwLock<State>> = Storage::new();
const SCREEN_WIDTH: f32 = 1500.0;
const SCREEN_HEIGHT: f32 = 900.0;

fn main() {
    let app_state = State {
        count: 0,
        incoming_move: None,
    };
    STATE.set(RwLock::new(app_state));

    let mut path;
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        path = std::path::PathBuf::from(manifest_dir.clone());
        path.push("resources");

        let conf = conf::WindowSetup::default()
            .title("Fog of War Chess")
            .vsync(false);

        let (mut ctx, mut event_loop) = ContextBuilder::new("Fog of war", "Isak & Hampus")
            .window_mode(
                conf::WindowMode::default()
                    .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
                    .maximized(false)
                    .resizable(false),
            )
            .window_setup(conf)
            .add_resource_path(path)
            .build()
            .expect("contextbuilder fail");
        graphics::set_drawable_size(&mut ctx, SCREEN_WIDTH, SCREEN_HEIGHT).expect("window drawable fail");
        graphics::set_screen_coordinates(&mut ctx, Rect::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT))
            .expect("screen coord fail");

        let mut game = Game::new(&mut ctx, true);

        let mut command_buffer = String::new();
        let mut payload_buffer = String::new();
        let stdin = io::stdin();

        command_buffer.clear();
        payload_buffer.clear();
        println!("Create or join? (c/j): ");
        stdin.read_line(&mut command_buffer).expect("Could not read line");

        if command_buffer.trim() == "c" {
            game.connection.send("create_room", "");
        } else {
            println!("Room code?: ");
            stdin.read_line(&mut payload_buffer).expect("Could not readline");
            game.connection.send("join_room", &payload_buffer);
            // If playing as black, since white starts
            game.active_turn = false;
            game.playing_as_white = false;
        }

        // Run!
        match event::run(&mut ctx, &mut event_loop, &mut game) {
            Ok(_) => {}
            Err(e) => println!("Error occured: {}", e),
        }
    } else {
        println!("Error loading file.");
    }
}
