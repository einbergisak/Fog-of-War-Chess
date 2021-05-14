use rust_socketio::{Payload, Socket};

use crate::{move_struct::Move, STATE, Room};


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
        Payload::String(str) => {
            println!("opponent connect: {}", str);
            STATE
                .get()
                .write()
                .unwrap()
                .event_validation
                .opponent_connect = true;
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_opponent_disconnect(payload: Payload, _: Socket) {
    match payload {
        Payload::String(str) => {
            println!("opponent disconnected: {}", str);
            STATE
                .get()
                .write()
                .unwrap()
                .event_validation
                .opponent_disconnect = true;
        }
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
        Payload::String(str) => {
            println!("join room: {}", str);
            if str == "true" {
                STATE.get().write().unwrap().event_validation.join_room = true;
            }
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_create_room(payload: Payload, _: Socket) {
    match payload {
        Payload::String(str) => {
            println!("create room: {}", str);
            if str != "false" {
                STATE.get().write().unwrap().event_validation.create_room = true;
            }
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_list_room(payload: Payload, _: Socket) {
    println!("Incoming package");
    match payload {
        Payload::String(str) => {
            let incoming: Vec<char> = str.chars().collect();
            let mut rooms: Vec<Room> = Vec::new();

            let mut current_id = String::from("");
            let mut current_members = String::from("");
            let mut id_active = true;

            for i in 0..str.len() {
                match &incoming[i].to_string()[..] {
                    "\"" => {
                        continue;
                    }
                    ":" => {
                        id_active = false;
                    }
                    ";" => {
                        rooms.push(Room {
                            id: current_id.to_string(),
                            members: current_members.parse::<i32>().unwrap(),
                        });
                        current_id = String::from("");
                        current_members = String::from("");
                        id_active = true;
                    }
                    letter => {
                        let temp_array: Vec<char> = letter.chars().collect();
                        if id_active {
                            current_id.push(temp_array[0]);
                        } else {
                            current_members.push(temp_array[0]);
                        }
                    }
                }
            }

            STATE.get().write().unwrap().lobbies = rooms;
            STATE.get().write().unwrap().lobby_sync += 1;
        }
        Payload::Binary(_) => {}
    }
}
