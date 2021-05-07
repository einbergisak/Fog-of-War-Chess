use rust_socketio::{Payload, Socket};

use crate::{APP_STATE};

pub(crate) fn on_opponent(payload: Payload, _: Socket) {
	let app_state = APP_STATE.get();
	println!("Accessed: {:?}", app_state.read().unwrap());
	app_state.write().unwrap().count += 1;

	match payload {
		Payload::String(str) => println!("on_opponent: {}", str),
	    Payload::Binary(_) => {}
	}
}

pub(crate) fn on_opponent_connect(payload: Payload, _: Socket) {
	match payload {
		Payload::String(str) => println!("opponent connect: {}", str),
	    Payload::Binary(_) => {}
	}
}

pub(crate) fn on_opponent_disconnect(payload: Payload, _: Socket) {
	match payload {
		Payload::String(str) => println!("opponent disconnected: {}", str),
	    Payload::Binary(_) => {}
	}
}

pub(crate) fn on_list_rooms(payload: Payload, _: Socket) {
	match payload {
		Payload::String(str) => println!("rooms: {}", str),
	    Payload::Binary(_) => {}
	}
}

pub(crate) fn on_join_room(payload: Payload, _: Socket) {
	match payload {
		Payload::String(str) => println!("join room: {}", str),
	    Payload::Binary(_) => {}
	}
}

pub(crate) fn on_create_room(payload: Payload, _: Socket) {
	match payload {
		Payload::String(str) => println!("create room: {}", str),
	    Payload::Binary(_) => {}
	}
}