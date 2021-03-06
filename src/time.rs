
use std::time::{Duration, Instant};

use ggez::{
    graphics::{self, DrawMode},
    Context,
};

use crate::{
    event_handler::{BOARD_ORIGO_X, BOARD_ORIGO_Y, BOARD_WIDTH},
    game::{Game, DARK_COLOR, LIGHT_COLOR},
    menu::clickable::ClickableGroup,
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub(crate) const TIME_TEXT_WIDTH: f32 = (SCREEN_WIDTH * 2.0 / 3.0 - 50.0) / 3.0;
pub(crate) const TIME_TEXT_HEIGHT: f32 = 50.0;
pub(crate) const TIME_BACKDROP_Y_OFFSET: f32 = 30.0;
pub(crate) const TIME_TITLE_Y_OFFSET: f32 = 150.0;
pub(crate) const TIME_TEXT_Y_POSITION: f32 = SCREEN_HEIGHT * 2.0 / 5.0;
pub(crate) const TIME_TEXT_PADDING: f32 = 100.0;

// Time is measured in seconds
pub(crate) struct Time {
    pub(crate) initial_time: Duration,
    pub(crate) current_time_left: Duration,
    pub(crate) opponent_time_left: Duration,
    pub(crate) turn_start: Instant,
    pub(crate) increment: Duration,
    pub(crate) time_set: bool,
}

impl Game {
    fn format_time(time_left: u64) -> String {
        let minutes = time_left / 60;
        let seconds = time_left % 60;

        let mut string = String::from("");
        if minutes < 10 {
            string.push_str(&format!("0{}", minutes)[..]);
        } else {
            string.push_str(&minutes.to_string()[..]);
        }

        string.push_str(":");

        if seconds < 10 {
            string.push_str(&format!("0{}", seconds)[..]);
        } else {
            string.push_str(&seconds.to_string()[..]);
        }
        return string;
    }

    pub(crate) fn render_time(&mut self, ctx: &mut Context) {
        if !self.active_turn {
            match graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                graphics::Rect::new(
                    BOARD_ORIGO_X + BOARD_WIDTH as f32 + 30.0,
                    BOARD_ORIGO_Y,
                    (SCREEN_WIDTH - BOARD_WIDTH as f32) / 2.0 - 60.0,
                    80.0,
                ),
                graphics::Color::from(DARK_COLOR),
            ) {
                Ok(drawable) => {
                    graphics::draw(ctx, &drawable, graphics::DrawParam::default())
                        .expect("Could not render your turn selection");
                }
                Err(_) => {}
            }
        }

        // Opponent time left
        let opponent_time = if self.game_active {
            if !self.active_turn && self.time.opponent_time_left >= self.time.turn_start.elapsed() {
                self.time.opponent_time_left - self.time.turn_start.elapsed()
            } else {
                self.time.opponent_time_left
            }
        } else {
            self.time.initial_time
        };
        self.menu.draw_text(
            ctx,
            Game::format_time(opponent_time.as_secs()),
            (BOARD_ORIGO_X + BOARD_WIDTH as f32, BOARD_ORIGO_Y + 20.0),
            ((SCREEN_WIDTH - BOARD_WIDTH as f32) / 2.0, 40.0),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );

        if self.active_turn {
            match graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                graphics::Rect::new(
                    BOARD_ORIGO_X + BOARD_WIDTH as f32 + 30.0,
                    BOARD_ORIGO_Y + BOARD_WIDTH as f32 - 80.0,
                    (SCREEN_WIDTH - BOARD_WIDTH as f32) / 2.0 - 60.0,
                    80.0,
                ),
                graphics::Color::from(DARK_COLOR),
            ) {
                Ok(drawable) => {
                    graphics::draw(ctx, &drawable, graphics::DrawParam::default())
                        .expect("Could not render your turn selection");
                }
                Err(_) => {}
            }
        }

        // User time left
        let time_left = if self.game_active {
            if self.active_turn && self.time.current_time_left >= self.time.turn_start.elapsed() {
                self.time.current_time_left - self.time.turn_start.elapsed()
            } else {
                self.time.current_time_left
            }
        } else {
            self.time.initial_time
        };
        self.menu.draw_text(
            ctx,
            Game::format_time(time_left.as_secs()),
            (
                BOARD_ORIGO_X + BOARD_WIDTH as f32,
                BOARD_ORIGO_Y + BOARD_WIDTH as f32 - 60.0,
            ),
            ((SCREEN_WIDTH - BOARD_WIDTH as f32) / 2.0, 40.0),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );
    }

    pub(crate) fn modify_time(&mut self, count: Duration, positive: bool, is_increment: bool) {
        if !self.is_admin {
            println!("This user is not an admin, thus does not have permission to edit the time");
            return;
        }

        if is_increment {
            if positive {
                if self.time.increment + count < Duration::from_secs(120) {
                    self.time.increment += count;
                }
            } else {
                if self.time.increment >= count {
                    self.time.increment -= count;
                }
            }
            return;
        }

        if positive {
            // Cannot have games longer than 2 hours
            if self.time.initial_time + count < Duration::from_secs(7200) {
                self.time.initial_time += count
            }
        } else {
            // Cannot have games with negative time
            if self.time.initial_time > count
                && self.time.initial_time - count >= Duration::from_secs(15)
            {
                self.time.initial_time -= count;
            }
        }
    }

    pub(crate) fn perform_time_increment(&mut self) {
        if self.active_turn && self.game_active {
            self.time.current_time_left -= self.time.turn_start.elapsed();
            self.time.current_time_left += self.time.increment;
        } else if self.game_active {
            self.time.opponent_time_left -= self.time.turn_start.elapsed();
            self.time.opponent_time_left += self.time.increment;
        }
    }

    pub(crate) fn render_time_interface(&mut self, ctx: &mut Context) {
        let minutes = self.time.initial_time.as_secs() / 60;
        let seconds = self.time.initial_time.as_secs() % 60;

        // Draw page title
        self.menu.draw_text(
            ctx,
            String::from("Chess clock"),
            (0.0, SCREEN_HEIGHT * 0.05),
            (SCREEN_WIDTH, TIME_TEXT_HEIGHT * 2.0),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );

        match graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0 - TIME_TEXT_PADDING - TIME_TEXT_WIDTH,
                TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET,
                TIME_TEXT_WIDTH,
                TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET * 2.0,
            ),
            graphics::Color::from(DARK_COLOR),
        ) {
            Ok(drawable) => {
                graphics::draw(ctx, &drawable, graphics::DrawParam::default())
                    .expect("Could not render background text");
            }
            Err(_) => {}
        }

        // Draw minutes title
        self.menu.draw_text(
            ctx,
            String::from("Minutes"),
            (
                SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0 - TIME_TEXT_PADDING - TIME_TEXT_WIDTH,
                TIME_TEXT_Y_POSITION - TIME_TITLE_Y_OFFSET,
            ),
            (TIME_TEXT_WIDTH, TIME_TEXT_HEIGHT),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );
        // Draw minutes
        self.menu.draw_text(
            ctx,
            minutes.to_string(),
            (
                SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0 - TIME_TEXT_PADDING - TIME_TEXT_WIDTH,
                TIME_TEXT_Y_POSITION,
            ),
            (TIME_TEXT_WIDTH, TIME_TEXT_HEIGHT),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );

        match graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0,
                TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET,
                TIME_TEXT_WIDTH,
                TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET * 2.0,
            ),
            graphics::Color::from(DARK_COLOR),
        ) {
            Ok(drawable) => {
                graphics::draw(ctx, &drawable, graphics::DrawParam::default())
                    .expect("Could not render background text");
            }
            Err(_) => {}
        }

        // Draw seconds title
        self.menu.draw_text(
            ctx,
            String::from("Seconds"),
            (
                SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0,
                TIME_TEXT_Y_POSITION - TIME_TITLE_Y_OFFSET,
            ),
            (TIME_TEXT_WIDTH, TIME_TEXT_HEIGHT),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );
        // Draw seconds
        self.menu.draw_text(
            ctx,
            seconds.to_string(),
            (
                SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0,
                TIME_TEXT_Y_POSITION,
            ),
            (TIME_TEXT_WIDTH, TIME_TEXT_HEIGHT),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );

        match graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                SCREEN_WIDTH / 2.0 + TIME_TEXT_WIDTH / 2.0 + TIME_TEXT_PADDING,
                TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET,
                TIME_TEXT_WIDTH,
                TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET * 2.0,
            ),
            graphics::Color::from(DARK_COLOR),
        ) {
            Ok(drawable) => {
                graphics::draw(ctx, &drawable, graphics::DrawParam::default())
                    .expect("Could not render background text");
            }
            Err(_) => {}
        }

        // Draw increment title
        self.menu.draw_text(
            ctx,
            String::from("Increment"),
            (
                SCREEN_WIDTH / 2.0 + TIME_TEXT_WIDTH / 2.0 + TIME_TEXT_PADDING,
                TIME_TEXT_Y_POSITION - TIME_TITLE_Y_OFFSET,
            ),
            (TIME_TEXT_WIDTH, TIME_TEXT_HEIGHT),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );
        // Draw increment
        self.menu.draw_text(
            ctx,
            self.time.increment.as_secs().to_string(),
            (
                SCREEN_WIDTH / 2.0 + TIME_TEXT_WIDTH / 2.0 + TIME_TEXT_PADDING,
                TIME_TEXT_Y_POSITION,
            ),
            (TIME_TEXT_WIDTH, TIME_TEXT_HEIGHT),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );

        self.menu
            .draw_clickables(ctx, vec![ClickableGroup::TimeSelection]);
    }
}
