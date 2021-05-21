use crate::{networking::events, piece::piece::PieceColor};
use rust_socketio::{Socket, SocketBuilder};
use serde_json::json;
pub(crate) struct Networking {
    socket: Socket,
}

#[derive(Debug, Clone)]
pub(crate) struct NetworkEventValidation {
    pub(crate) create_room: bool,
    pub(crate) join_room: bool,
    pub(crate) opponent_connect: bool,
    pub(crate) opponent_disconnect: bool,
    pub(crate) play_again: bool,
    pub(crate) set_color: Option<PieceColor>,
    pub(crate) resign: bool,
    pub(crate) opponent_name: Option<String>,
    pub(crate) time: Option<(i32, i32)>,
    pub(crate) deselect_cursor: bool
}

#[derive(Debug, Clone)]
pub(crate) struct Room {
    pub(crate) id: String,
    pub(crate) members: i32,
}

impl Networking {
    pub(crate) fn new() -> Networking {
        let socket = SocketBuilder::new("http://chess.datasektionen.link")
            .set_namespace("/")
            .expect("illegal namespace")
            .on("join_room_res", |payload, socket| {
                events::on_join_room(payload, socket)
            })
            .on("create_room_res", |payload, socket| {
                events::on_create_room(payload, socket)
            })
            .on("opponent", |payload, socket| {
                events::on_opponent(payload, socket)
            })
            .on("opponent_connect", |payload, socket| {
                events::on_opponent_connect(payload, socket)
            })
            .on("opponent_disconnect", |payload, socket| {
                events::on_opponent_disconnect(payload, socket)
            })
            .on("opponent_leave_lobby", |payload, socket| {
                events::on_opponent_disconnect(payload, socket)
            })
            .on("list_rooms", |payload, socket| {
                events::on_list_room(payload, socket)
            })
            .on("play_again", |payload, socket| {
                events::on_play_again(payload, socket)
            })
            .on("set_opponent_color", |payload, socket| {
                events::on_set_opponent_color(payload, socket)
            })
            .on("resign", |payload, socket| {
                events::on_resign(payload, socket)
            })
            .on("get_opponent_name", |payload, socket| {
                events::on_get_opponent_name(payload, socket)
            })
            .on("set_clock_time", |payload, socket| {
                events::on_set_clock_time(payload, socket)
            })
            .on("error", |err, _| eprintln!("Error: {:#?}", err))
            .connect()
            .expect("Connection failed");

        return Networking { socket };
    }

    pub(crate) fn send(&mut self, event: &str, data: &str) {
        println!("SENDING '{}' '{}'", event.trim(), data.trim());
        self.socket
            .emit(event.trim(), json!(data.trim()))
            .expect("Could not emit payload via sockets");
    }
}

/* pub(crate) fn establish_connection() {
    // define a callback which is called when a payload is received
    // this callback gets the payload as well as an instance of the
    // socket to communicate with the server
    let callback = |payload: Payload, mut socket: Socket| {
        match payload {
            Payload::String(str) => println!("Received: {}", str),
            Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
        }
        socket.emit("test", json!({"got ack": true})).expect("Server unreachable")
    };

    // get a socket that is connected to the admin namespace


    // emit to the "foo" event
    let json_payload = json!({"token": 123});

    socket.emit("list_rooms", json_payload).expect("Server unreachable");

    // define a callback, that's executed when the ack got acked
    let ack_callback = |message: Payload, _| {
        println!("Yehaa! My ack got acked?");
        println!("Ack data: {:#?}", message);
    };

    let json_payload = json!({"myAckData": 123});

    // emit with an ack
    let ack = socket
        .emit_with_ack("test", json_payload, Duration::from_secs(2), ack_callback)
        .expect("Server unreachable");
} */
