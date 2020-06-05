//! Implement behaviour of `ChatServer`.
//!
//! [`ChaServer`] is a map, store the relation ship between id, room id and real entity.
//!
//!

use super::*;
use crate::messages::DanmakuMessage;

impl ChatServer {

    fn broadcast_danmaku(&self, room: &str, danmaku: DanmakuMessage,skip_id: usize){
        println!("broadcast: {:?}", danmaku);
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(danmaku.clone());
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
    //                     let _ = addr.do_send(messages::Message(message.to_owned()));
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

impl Handler<messages::DanmakuMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, danmaku: messages::DanmakuMessage, _: &mut Self::Context) {
        // do some checks before broadcast.
        // 根据 msg 本身进行过滤。
        // let data = msg.ned();
        // TODO: currently working on.
        // TODO: do the check
        let skip_id = danmaku.id;
        let room = danmaku.room.to_owned();
        self.broadcast_danmaku(&room, danmaku, skip_id);
    }
}

trait TempTest {

}

impl Handler<messages::Connect> for ChatServer {
    /// id
    type Result = usize;

    fn handle(&mut self, msg: messages::Connect, _: &mut Self::Context) -> Self::Result {
        println!("someone joined");

        // notify all users in same room
        // self.broadcast_message(&msg.room, "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // join session
        let s = self.rooms.entry(msg.room).or_default();
        s.insert(id);

        // send new id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<messages::Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: messages::Disconnect, _: &mut Self::Context) {
        println!("Someone disconnect");

        // remove address
        self.sessions.remove(&msg.id);

        // remove room info
        // free memory when no one left in room.
        self.rooms.get_mut(&msg.room).unwrap().remove(&msg.id);
    }
}
