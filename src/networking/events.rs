use rust_socketio::{Payload, Socket};

use crate::{STATE, game::Move};

pub(crate) fn on_opponent(payload: Payload, _: Socket) {
    let app_state = STATE.get();

    match payload {
        Payload::String(string) => {
            println!("Incoming move: {}", string);
            app_state.write().unwrap().incoming_move = Some(Move::from_str(string));

        }
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
