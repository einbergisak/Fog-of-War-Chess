use ggez::{Context, graphics};

use crate::{SCREEN_WIDTH, STATE, event_handler::{BOARD_ORIGO_X, BOARD_ORIGO_Y, BOARD_WIDTH}, game::{Game, LIGHT_COLOR}, piece::piece::PieceColor};

// Time is measured in seconds
pub(crate) struct Time {
    pub(crate) clock: i32,
    pub(crate) time_left: i32,
    pub(crate) opponent_time_left: i32,
    pub(crate) total_time: i32
}

impl Game {
	pub(crate) fn run_clock(&mut self) {

		// If no opponent has connected or the game hasn't started we don't decrease the time
		if !STATE.get().read().unwrap().opponent_online || !self.game_active {
			return
		}

		// Decrease clock until we reach one second
		if self.time.clock < 60 {
			self.time.clock += 1;
			return
		} 

		// Reset clock
		self.time.clock = 0;
		
		if self.active_turn {
			self.time.time_left -= 1;
		} else {
			self.time.opponent_time_left -= 1;
		}

		if self.time.time_left <= 0 {
			if self.playing_as_white {
				self.game_over(PieceColor::Black);
			} else {
				self.game_over(PieceColor::White);
			}
		} else if self.time.opponent_time_left <= 0 {
			if self.playing_as_white {
				self.game_over(PieceColor::White);
			} else {
				self.game_over(PieceColor::Black);
			}
		}
	}

	fn format_time(time_left: i32) -> String {
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

		// Opponent time left
		self.menu.draw_text(
			ctx, 
			Game::format_time(self.time.opponent_time_left), 
			(
				BOARD_ORIGO_X + BOARD_WIDTH as f32,
				BOARD_ORIGO_Y
			), 
			(
				(SCREEN_WIDTH - BOARD_WIDTH as f32) / 2.0,
				40.0
			), 
			graphics::Color::from(LIGHT_COLOR), 
			graphics::Align::Center
		);

		// User time left
		self.menu.draw_text(
			ctx, 
			Game::format_time(self.time.time_left), 
			(
				BOARD_ORIGO_X + BOARD_WIDTH as f32,
				BOARD_ORIGO_Y + BOARD_WIDTH as f32 - 40.0
			), 
			(
				(SCREEN_WIDTH - BOARD_WIDTH as f32) / 2.0,
				40.0
			), 
			graphics::Color::from(LIGHT_COLOR), 
			graphics::Align::Center
		);
	}
}