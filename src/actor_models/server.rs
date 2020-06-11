//! Implement behaviour of `ChatServer`.
//!
//! [`ChaServer`] is a map, store the relation ship between id, room id and real entity.
//!
//!

use super::*;

impl ChatServer {
    fn broadcast(&self, room: &str, server_message: ServerMessage, skip_id: usize) {
        println!("broadcast: {:?}", server_message);
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(server_message.clone());
                    }
                }
            }
        }
    }

    // / Send message to all users in the room.
    // /
    // / # Arguments
    // /
    // / * `room`: room name
    // /
    // / #
    // fn broadcast_message(&self, room: &str, message: &str, skip_id: usize) {
    //     if let Some(sessions) = self.rooms.get(room) {
    //         for id in sessions {
    //             if *id != skip_id {
    //                 if let Some(addr) = self.sessions.get(id) {
    //                     let _ = addr.do_send(Message(message.to_owned()));
    //                 }
    //             }
    //         }
    //     }
    // }
    // todo: broadcast monitor message

    // todo: hb to send monitor data
}

impl Default for ChatServer {
    fn default() -> Self {
        let mut rooms = HashMap::new();
        rooms.insert("Default".into(), HashSet::new());

        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<DanmakuMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, danmaku: DanmakuMessage, _: &mut Self::Context) {
        // do some checks before broadcast.
        // 根据 msg 本身进行过滤。
        // let data = msg.ned();
        // TODO: log

        let skip_id = danmaku.id;
        let room = danmaku.room.to_owned();
        self.broadcast(&room, ServerMessage::from(danmaku), skip_id);
    }
}

trait TempTest {}

impl Handler<Connect> for ChatServer {
    /// id
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        println!("someone joined");

        // notify all users in same room
        // self.broadcast_message(&msg.room, "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // join session
        let s = self.rooms.entry(msg.room.to_owned()).or_default();
        s.insert(id);
        let room_size = s.len();

        self.broadcast(&msg.room, ServerMessage::StatisticInfo(room_size), 0);
        // send new id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        println!("Someone disconnect");

        // remove address
        self.sessions.remove(&msg.id);

        // remove room info
        // free memory when no one left in room.
        let room = self.rooms.get_mut(&msg.room).unwrap();
        room.remove(&msg.id);

        // broadcast
        let room_size = room.len();
        self.broadcast(&msg.room, ServerMessage::StatisticInfo(room_size), 0);
    }
}
