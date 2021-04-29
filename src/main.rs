use ggez::{Context, ContextBuilder, GameResult, conf, graphics::{self, Drawable, Rect}, input::mouse};
use ggez::event::{self, EventHandler};

fn main() {
    let mut path;
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        loop {
            path = std::path::PathBuf::from(manifest_dir.clone());
            path.push("resources");

            let (mut ctx, mut event_loop) = ContextBuilder::new("Fog of war", "Isak & Hampus")
                .window_mode(
                    conf::WindowMode::default()
                        .dimensions(1920.0, 1080.0)
                        .maximized(false)
                        .resizable(true)
                )
                .add_resource_path(path)
                .build()
                .expect("contextbuilder fail");
            graphics::set_drawable_size(&mut ctx, 1920.0, 1080.0).expect("window drawable fail");
            graphics::set_screen_coordinates(&mut ctx, Rect::new(0.0, 0.0, 1920.0, 1080.0))
                .expect("screen coord fail");

            let mut game = Game::new(&mut ctx);
            // Run!
            let mut k: Option<bool> = Some(true);
            match k {
                std::option::Option::Some(boolean) => {

                }
                std::option::Option::None => {}
            }
            match event::run(&mut ctx, &mut event_loop, &mut game) {
                Ok(_) => {
                }
                Err(e) => println!("Error occured: {}", e),
            }
        }
    } else {
        println!("Error loading file.");
    }
}

struct Game {
    // Your state here...
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        // Load/create resources such as images here.
        Game {
            // ...
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        let squareSize = 20;
        for row in 0..8 {
            for column in 0..8 {
                let mut rect = Rect::new_i32(column * squareSize, row * squareSize, squareSize, squareSize);

                DrawParam::new()
                        .src(src)
                        .scale(Vector2::new(
                            self.scaled_tile_size / DEFAULT_TILE_SIZE,
                            self.scaled_tile_size / DEFAULT_TILE_SIZE,
                        ))
                        .dest(Point2::new(
                            x as f32 * self.scaled_tile_size,
                            y as f32 * self.scaled_tile_size,
                        ))
                graphics::draw(ctx, &rect, test)
            }
        }
        graphics::present(ctx)
    }
}