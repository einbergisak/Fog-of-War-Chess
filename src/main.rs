use default_board_state::generate_default_board;
use ggez::event::{self};
use ggez::{
    conf,
    graphics::{self, Rect},
    Context, ContextBuilder,
};
use piece::{Board, Piece};
mod default_board_state;
mod event_handler;
mod piece;
mod render_utilities;

fn main() {
    let mut path;
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        path = std::path::PathBuf::from(manifest_dir.clone());
        path.push("resources");

        let conf = conf::WindowSetup::default().title("Fog of War Chess").vsync(false);

        let (mut ctx, mut event_loop) = ContextBuilder::new("Fog of war", "Isak & Hampus")
            .window_mode(
                conf::WindowMode::default()
                    .dimensions(800.0, 800.0)
                    .maximized(false)
                    .resizable(false),
            )
            .window_setup(conf)
            .add_resource_path(path)
            .build()
            .expect("contextbuilder fail");
        graphics::set_drawable_size(&mut ctx, 800.0, 800.0).expect("window drawable fail");
        graphics::set_screen_coordinates(&mut ctx, Rect::new(0.0, 0.0, 800.0, 800.0))
            .expect("screen coord fail");

        let mut game = Game::new(&mut ctx);

        // Run!
        match event::run(&mut ctx, &mut event_loop, &mut game) {
            Ok(_) => {}
            Err(e) => println!("Error occured: {}", e),
        }
    } else {
        println!("Error loading file.");
    }
}

// Main struct
struct Game {
    board: Board,
    grabbed_piece: Option<(Piece, (usize, usize))>,
    flipped_board: bool
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {
            board: generate_default_board(), // Load/create resources such as images here.
            grabbed_piece: None,
            flipped_board: true
        }
    }
}
