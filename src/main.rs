use ggez::{event::{self, EventHandler}, graphics::{Drawable}};
use ggez::{
    conf,
    graphics::{
        self, spritebatch::SpriteBatch, Color, DrawMode, DrawParam, Image, MeshBuilder, Rect,
    },
    nalgebra::{Point2, Vector2},
    Context, ContextBuilder, GameResult,
};
use piece::{Board, Piece, PieceType::*};
mod piece;

const BOARD_SIZE: usize = 8;

fn main() {
    let mut path;
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        path = std::path::PathBuf::from(manifest_dir.clone());
        path.push("resources");

        let (mut ctx, mut event_loop) = ContextBuilder::new("Fog of war", "Isak & Hampus")
            .window_mode(
                conf::WindowMode::default()
                    .dimensions(800.0, 800.0)
                    .maximized(false)
                    .resizable(false),
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

        let mut board = vec![None; 64];

        // White pieces
        board[0] = Some(Piece{piece_type: Rook, color: piece::Color::White});
        board[1] = Some(Piece{piece_type: Knight, color: piece::Color::White});
        board[2] = Some(Piece{piece_type: Bishop, color: piece::Color::White});
        board[3] = Some(Piece{piece_type: King, color: piece::Color::White});
        board[4] = Some(Piece{piece_type: Queen, color: piece::Color::White});
        board[5] = Some(Piece{piece_type: Bishop, color: piece::Color::White});
        board[6] = Some(Piece{piece_type: Knight, color: piece::Color::White});
        board[7] = Some(Piece{piece_type: Rook, color: piece::Color::White});

        // White pawns
        for i in 8..16{
            board[i] = Some(Piece{piece_type: Pawn, color: piece::Color::White});
        }

        // Black pawns
        for i in 48..56{
            board[i] = Some(Piece{piece_type: Pawn, color: piece::Color::Black});
        }

        // Black pieces
        board[56] = Some(Piece{piece_type: Rook, color: piece::Color::Black});
        board[57] = Some(Piece{piece_type: Knight, color: piece::Color::Black});
        board[58] = Some(Piece{piece_type: Bishop, color: piece::Color::Black});
        board[59] = Some(Piece{piece_type: King, color: piece::Color::Black});
        board[60] = Some(Piece{piece_type: Queen, color: piece::Color::Black});
        board[61] = Some(Piece{piece_type: Bishop, color: piece::Color::Black});
        board[62] = Some(Piece{piece_type: Knight, color: piece::Color::Black});
        board[63] = Some(Piece{piece_type: Rook, color: piece::Color::Black});


        Game {
            board,
        }
    }
}

// Translates from coordinates to list index
fn translate(x: usize, y: usize) -> usize {
    return (y - 1) * BOARD_SIZE + x - 1;
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
        graphics::draw(ctx, &mesh, (Point2::<f32>::new(0.0, 0.0),))?;

        let piece_image = Image::new(ctx, "/pieces.png")?;
        let mut piece_batch = SpriteBatch::new(piece_image);

        for (index, tile) in self.board.iter().enumerate() {
            match tile {
                Some(piece) => {
                    let src_image_y = match piece.color {
                        piece::Color::White => 0.0,
                        piece::Color::Black => 0.5,
                    };
                    let src_image_x = match piece.piece_type {
                        King => 0.0,
                        Queen => 1.0 / 6.0,
                        Bishop => 2.0 / 6.0,
                        Knight => 3.0 / 6.0,
                        Rook => 4.0 / 6.0,
                        Pawn => 5.0 / 6.0,
                    };
                    let rect = Rect::new(src_image_x, src_image_y, 1.0/6.0, 0.5);
                    let y = index / BOARD_SIZE;
                    let x = index % BOARD_SIZE;
                    let mut param = DrawParam::default()
                        .src(rect)
                        .dest(Point2::new((x as f32) * tile_size as f32 , (y as f32) * tile_size as f32 ));
                    piece_batch.add(param);
                }
                None => {}
            }
        }
        graphics::draw(ctx, &piece_batch, (Point2::<f32>::new(0.0, 0.0),))?;

        // Draw pieces
        //let mut piece_batch = SpriteBatch::new()

        graphics::present(ctx)
    }
}
