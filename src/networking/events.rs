use rust_socketio::{Payload, Socket};

use crate::{move_struct::Move, piece::piece::PieceColor, Room, STATE};

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

            STATE.get().write().unwrap().opponent_online = true;
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_opponent_disconnect(payload: Payload, _: Socket) {
    match payload {
        Payload::String(_) => {
            STATE
                .get()
                .write()
                .unwrap()
                .event_validation
                .opponent_disconnect = true;

            STATE.get().write().unwrap().opponent_online = false;
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_join_room(payload: Payload, _: Socket) {
    match payload {
        Payload::String(str) => {
            println!("join room: {}", str);
            if str == "true" {
                STATE.get().write().unwrap().event_validation.join_room = true;
            } else {
                // If we failed to join the lobby we remove the room id again
                STATE.get().write().unwrap().room_id = None;
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
                STATE.get().write().unwrap().room_id = Some(str);
            }
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_list_room(payload: Payload, _: Socket) {
    match payload {
        Payload::String(str) => {
            println!("Got new list rooms");
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

pub(crate) fn on_play_again(payload: Payload, _: Socket) {
    match payload {
        Payload::String(_) => {
            println!("Received play again, changing STATE!");
            STATE.get().write().unwrap().event_validation.play_again = true;
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_set_opponent_color(payload: Payload, _: Socket) {
    match payload {
        Payload::String(mut str) => {
            str = str.replace("\"", "");
            if str == "white" {
                STATE.get().write().unwrap().event_validation.set_color = Some(PieceColor::White);
            } else {
                STATE.get().write().unwrap().event_validation.set_color = Some(PieceColor::Black);
            }
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_resign(payload: Payload, _: Socket) {
    match payload {
        Payload::String(_) => {
            STATE.get().write().unwrap().event_validation.resign = true;
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_get_opponent_name(payload: Payload, _: Socket) {
    match payload {
        Payload::String(opponent_name) => {
            STATE.get().write().unwrap().event_validation.opponent_name =
                Some(opponent_name.replace("\"", ""));
        }
        Payload::Binary(_) => {}
    }
}

pub(crate) fn on_set_clock_time(payload: Payload, _: Socket) {
    match payload {
        Payload::String(package) => {
            let data = package.replace("\"", "");
            let mut split = data.split(":");
            let total_time = split.next().unwrap().parse::<u64>().unwrap();
            let increment = split.next().unwrap().parse::<u64>().unwrap();

            STATE.get().write().unwrap().event_validation.time = Some((total_time, increment));
        }
        Payload::Binary(_) => {}
    }
}
