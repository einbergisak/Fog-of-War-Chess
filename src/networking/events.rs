use rust_socketio::{Payload, Socket};

use crate::{STATE, networking::connection::Room};

pub(crate) fn on_opponent(payload: Payload, _: Socket) {
    let app_state = STATE.get();

    match payload {
        Payload::String(str) => {
            println!("{}", str);

            let mut content = str.chars();
            content.next();
            content.next_back();
            let moves: Vec<&str> = content.as_str().split(":").collect();
            println!("moves: {:?}", moves);

            let mut target_index = 0;
            let mut move_index = 0;

            let mut valid_count = 0;
            match moves[0].parse::<i32>() {
                Ok(index) => {
                    println!("Parsed {} on check 1", index);
                    target_index = index;
                    valid_count += 1;
                }
                Err(_) => {}
            }

            match moves[1].parse::<i32>() {
                Ok(index) => {
                    println!("Parsed {} on check 2", index);
                    move_index = index;
                    valid_count += 1;
                }
                Err(_) => {}
            }

            if valid_count == 2 {
                app_state.write().unwrap().incoming_move =
                    Some((target_index as usize, move_index as usize));
            } else {
                println!("Invalid incoming package: {}", str)
            }
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

pub(crate) fn on_list_room(payload: Payload, _: Socket) {
    println!("Incoming package");
    match payload {
        Payload::String(str) => {
            let incoming: Vec<char> = str.chars().collect();
            let mut rooms: Vec<Room> = Vec::new();

            let mut currentId = String::from("");
            let mut currentMembers = String::from("");
            let mut idActive = true;

            for i in 0..str.len() {
                match &incoming[i].to_string()[..] {
                    ":" => {
                        idActive = false;
                    }
                    ";" => {
                        rooms.push(Room {
                            id: currentId.to_string(),
                            members: currentMembers.parse::<i32>().unwrap()
                        });
                        currentId = String::from("");
                        currentMembers = String::from("");
                        idActive = true;
                    }
                    letter => {
                        let temp_array: Vec<char> = letter.chars().collect();
                        if idActive {
                            currentId.push(temp_array[0]);
                        } else {
                            currentMembers.push(temp_array[0]);
                        }
                    }
                }
            }

            STATE.get().write().unwrap().lobbies = rooms;
            STATE.get().write().unwrap().lobby_sync += 1;
        },
        Payload::Binary(_) => {}
    }
}
