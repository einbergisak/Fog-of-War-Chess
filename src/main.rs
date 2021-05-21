use std::sync::RwLock;

use game::Game;
use ggez::event;
use ggez::{
    conf,
    graphics::{self, Rect},
    ContextBuilder,
};
use move_struct::Move;
use networking::connection::{NetworkEventValidation, Room};
use state::Storage;

mod default_board_state;
mod event_handler;
mod game;
mod move_struct;
mod time;
mod piece {
    pub mod piece;
    pub mod piece_movement;
    pub mod promotion;
}

pub mod enter_name_screen;
mod render_utilities;
mod networking {
    pub mod connection;
    pub mod events;
}
mod menu {
    pub mod clickable;
    pub mod menu_game_over;
    pub mod menu_state;
    pub mod menu_utilities;
    pub mod create_clickable_layout;
}

#[derive(Debug, Clone)]
pub(crate) struct State {
    pub(crate) entering_name: bool,
    pub(crate) name: String,
    pub(crate) lobbies: Vec<Room>,
    pub(crate) lobby_sync: i32,
    pub(crate) event_validation: NetworkEventValidation,
    pub(crate) incoming_move: Option<Move>,
    pub(crate) room_id: Option<String>,
    pub(crate) opponent_online: bool,
}

static STATE: Storage<RwLock<State>> = Storage::new();
const SCREEN_WIDTH: f32 = 1500.0;
const SCREEN_HEIGHT: f32 = 900.0;

fn main() {
    let app_state = State {
        entering_name: true,
        name: String::from(""),
        incoming_move: None,
        lobbies: Vec::new(),
        lobby_sync: 0,
        event_validation: NetworkEventValidation {
            create_room: false,
            join_room: false,
            opponent_connect: false,
            opponent_disconnect: false,
            play_again: false,
            set_color: None,
            resign: false,
            opponent_name: None,
            time: None,
            deselect_cursor: false
        },
        room_id: None,
        opponent_online: false
    };
    STATE.set(RwLock::new(app_state));

    let mut path;
    if let Ok(manifest_dir) = std::env::current_exe() {
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
        graphics::set_drawable_size(&mut ctx, SCREEN_WIDTH, SCREEN_HEIGHT)
            .expect("window drawable fail");
        graphics::set_screen_coordinates(
            &mut ctx,
            Rect::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT),
        )
        .expect("screen coord fail");

        let mut game = Game::new(&mut ctx);

        game.connection.send("list_rooms", "");

        // Run!
        match event::run(&mut ctx, &mut event_loop, &mut game) {
            Ok(_) => {}
            Err(e) => println!("Error occured: {}", e),
        }
    } else {
        println!("Error loading file.");
    }
}
