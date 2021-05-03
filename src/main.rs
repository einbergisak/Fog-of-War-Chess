use ggez::event::{self, EventHandler};
use ggez::{
    conf,
    graphics::{self, Color, DrawMode, Drawable, MeshBuilder, Rect, DrawParam},
    input::mouse,
    Context, ContextBuilder, GameResult,
    nalgebra::Point2
};
use piece::Board;
mod piece;

fn main() {
    let mut path;
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        loop {
            path = std::path::PathBuf::from(manifest_dir.clone());
            path.push("resources");

            let (mut ctx, mut event_loop) = ContextBuilder::new("Fog of war", "Isak & Hampus")
                .window_mode(
                    conf::WindowMode::default()
                        .dimensions(800.0, 800.0)
                        .maximized(false)
                        .resizable(true),
                )
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
        }
    } else {
        println!("Error loading file.");
    }
}

// Main struct
struct Game {
    board: Board,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        // Load/create resources such as images here.
        Game {
            board: vec![None; 64],
        }
    }
}

// Translates from coordinates to list index
fn translate(x: usize, y: usize) -> usize {
    return (y - 1) * 8 + x - 1;
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }
    // y * 8 + x
    // (y-1) * 8 + x - 1
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let tile_size = 100;
        let dark_color: (u8, u8, u8) = (181, 136, 99);
        let light_color: (u8, u8, u8) = (240, 217, 181);

        let mut mesh_builder = MeshBuilder::new();

        let get_rect = |x_index: i32, y_index: i32| {
            return Rect::new_i32(
                x_index * tile_size,
                y_index * tile_size,
                tile_size,
                tile_size,
            );
        };
        // Calculate sprite batch
        let squareSize = 20;
        for row in 0..8 {
            for column in 0..8 {
                let color: (u8, u8, u8);
                if (column + row) % 2 == 0 {
                    // White
                    color = light_color;
                } else {
                    color = dark_color;
                }

                // Create Rectangle in mesh at position
                mesh_builder.rectangle(DrawMode::fill(), get_rect(column, row), Color::from(color));
            }
        }
        let mesh = mesh_builder.build(ctx)?;
        // let draw_param = DrawParam::new().dest(Point2::new_i32(0, 0));
        graphics::draw(ctx, &mesh, (Point2::<f32>::new(0.0, 0.0),));
        graphics::present(ctx)
    }
}
