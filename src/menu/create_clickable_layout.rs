use ggez::{graphics::Color};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, event_handler::BOARD_WIDTH, game::{ERROR_COLOR, LIGHT_COLOR}, time::{TIME_BACKDROP_Y_OFFSET, TIME_TEXT_HEIGHT, TIME_TEXT_PADDING, TIME_TEXT_WIDTH, TIME_TEXT_Y_POSITION}};

use super::{clickable::{Clickable, ClickableGroup, Transform}, menu_state::Menu};

impl Menu {
	pub(crate) fn create_clickables(&mut self) {
		// Main menu buttons ###########################################
		self.clickables.push(Clickable {
            id: String::from("create_room_button"),
            transform: Transform {
                x: SCREEN_WIDTH as i32 / 4 - 500 / 2,
                y: SCREEN_HEIGHT as i32 / 2 - 200 / 2,
                width: 500,
                height: 200,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            text: String::from("Create room"),
            list_item: false,
            group: ClickableGroup::MainMenu,
        });

        let board_right_edge = SCREEN_WIDTH / 2.0 + (BOARD_WIDTH / 2) as f32;

		// In game buttons ##############################################
        // Resign button for in game
        self.clickables.push(Clickable {
            id: String::from("resign_game_button"),
            transform: Transform {
                x: (board_right_edge + (SCREEN_WIDTH - board_right_edge) / 2.0 - 125.0 / 2.0)
                    as i32,
                y: (SCREEN_HEIGHT / 2.0 - 25.0) as i32,
                width: 125,
                height: 50,
            },
            color: Color::from(ERROR_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("Resign"),
            group: ClickableGroup::InGame,
        });

		// Name screen button ###########################################
        // Submit name button
        self.clickables.push(Clickable {
            id: String::from("submit_name_button"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - 150.0) as i32,
                y: (SCREEN_HEIGHT * 3.0 / 4.0 - 125.0 / 2.0) as i32,
                width: 300,
                height: 125,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("Submit name"),
            group: ClickableGroup::EnterName,
        });

		// Set time buttons ##############################################

		let button_width = 80.0;
		let button_height = 40.0;
		let button_padding = 20.0;

		// ### MINUTES ###
		self.clickables.push(Clickable {
            id: String::from("minute_plus_1"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0 - TIME_TEXT_PADDING - TIME_TEXT_WIDTH / 2.0 - button_width - button_padding - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET - button_height) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("+1"),
            group: ClickableGroup::TimeSelection,
        });
		self.clickables.push(Clickable {
            id: String::from("minute_plus_5"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0 - TIME_TEXT_PADDING - TIME_TEXT_WIDTH / 2.0 - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET - button_height) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("+5"),
            group: ClickableGroup::TimeSelection,
        });
		self.clickables.push(Clickable {
            id: String::from("minute_plus_10"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0 - TIME_TEXT_PADDING - TIME_TEXT_WIDTH / 2.0 + button_width + button_padding - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET - button_height) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("+10"),
            group: ClickableGroup::TimeSelection,
        });

		self.clickables.push(Clickable {
            id: String::from("minute_minus_1"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0 - TIME_TEXT_PADDING - TIME_TEXT_WIDTH / 2.0 - button_width - button_padding - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION + TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("-1"),
            group: ClickableGroup::TimeSelection,
        });
		self.clickables.push(Clickable {
            id: String::from("minute_minus_5"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0 - TIME_TEXT_PADDING - TIME_TEXT_WIDTH / 2.0 - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION + TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("-5"),
            group: ClickableGroup::TimeSelection,
        });
		self.clickables.push(Clickable {
            id: String::from("minute_minus_10"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - TIME_TEXT_WIDTH / 2.0 - TIME_TEXT_PADDING - TIME_TEXT_WIDTH / 2.0 + button_width + button_padding - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION + TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("-10"),
            group: ClickableGroup::TimeSelection,
        });

		// ### Seconds ###
		self.clickables.push(Clickable {
            id: String::from("second_plus_15"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET - button_height) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("+15"),
            group: ClickableGroup::TimeSelection,
        });

		self.clickables.push(Clickable {
            id: String::from("second_minus_15"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION + TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("-15"),
            group: ClickableGroup::TimeSelection,
        });

		// ### Increment ###
		self.clickables.push(Clickable {
            id: String::from("increment_plus_1"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 + TIME_TEXT_WIDTH / 2.0 + TIME_TEXT_PADDING + TIME_TEXT_WIDTH / 2.0 - button_width - button_padding - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET - button_height) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("+1"),
            group: ClickableGroup::TimeSelection,
        });
		self.clickables.push(Clickable {
            id: String::from("increment_plus_5"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 + TIME_TEXT_WIDTH / 2.0 + TIME_TEXT_PADDING + TIME_TEXT_WIDTH / 2.0 - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET - button_height) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("+5"),
            group: ClickableGroup::TimeSelection,
        });
		self.clickables.push(Clickable {
            id: String::from("increment_plus_10"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 + TIME_TEXT_WIDTH / 2.0 + TIME_TEXT_PADDING + TIME_TEXT_WIDTH / 2.0 + button_width + button_padding - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION - TIME_BACKDROP_Y_OFFSET - button_height) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("+10"),
            group: ClickableGroup::TimeSelection,
        });

		self.clickables.push(Clickable {
            id: String::from("increment_minus_1"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 + TIME_TEXT_WIDTH / 2.0 + TIME_TEXT_PADDING + TIME_TEXT_WIDTH / 2.0 - button_width - button_padding - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION + TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("-1"),
            group: ClickableGroup::TimeSelection,
        });
		self.clickables.push(Clickable {
            id: String::from("increment_minus_5"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 + TIME_TEXT_WIDTH / 2.0 + TIME_TEXT_PADDING + TIME_TEXT_WIDTH / 2.0 - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION + TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("-5"),
            group: ClickableGroup::TimeSelection,
        });
		self.clickables.push(Clickable {
            id: String::from("increment_minus_10"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 + TIME_TEXT_WIDTH / 2.0 + TIME_TEXT_PADDING + TIME_TEXT_WIDTH / 2.0 + button_width + button_padding - button_width / 2.0) as i32,
                y: (TIME_TEXT_Y_POSITION + TIME_TEXT_HEIGHT + TIME_BACKDROP_Y_OFFSET) as i32,
                width: button_width as i32,
                height: button_height as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("-10"),
            group: ClickableGroup::TimeSelection,
        });

		// Start game button
		self.clickables.push(Clickable {
            id: String::from("finish_time_start_game"),
            transform: Transform {
                x: (SCREEN_WIDTH / 2.0 - 300.0 / 2.0) as i32,
                y: (SCREEN_HEIGHT * 0.8) as i32,
                width: 300.0 as i32,
                height: 120.0 as i32,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            list_item: false,
            text: String::from("Enter game"),
            group: ClickableGroup::TimeSelection,
        });
	}
}