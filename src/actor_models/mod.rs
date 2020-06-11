//! A module stored actor models.
//!
//! The `actor_models` module contains all the things that actor needs.
//!
//! Two main actor model is [`ChatServer`] and [`WsChatSession`].

use actix_web_actors::ws;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::{HashMap, HashSet};

use crate::message_processor::MessageProcessor;
use actix::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::{Duration, Instant};

/// `ChatServer` handle all the connections, messages and disconnections.
/// There is only one ChatServer at a time.
pub struct ChatServer {
    pub sessions: HashMap<usize, Recipient<ServerMessage>>,
    pub rooms: HashMap<String, HashSet<usize>>,
    pub rng: ThreadRng,
    // pub monitors: HashMap<usize, Context<WsChatSession>>,
}

/// A `WsChatSession` is a map of websocket in server (normally, it means one user).
pub struct WsChatSession {
    /// unique session id
    pub id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,
    /// joined room
    pub room: String,
    /// peer name
    pub identity: Identity,
    /// chat server address
    pub addr: Addr<ChatServer>,
    pub message_processor: MessageProcessor,
}

/// Role of the session.
#[derive(PartialEq, Debug)]
pub enum Identity {
    /// `Anonymous` can't send danmaku when `REQUIRED_LOGIN` set `true`.
    Anonymous,
    /// `User(Name)` can send danmaku.
    User(String),
    /// `Admin(Name)` can send extra command info,
    /// or receive statistic status.
    Admin(String),
}

/// DPlayer compatible Danmaku info
#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Danmaku {
    #[serde(default)]
    pub user: String,
    pub text: String,
    pub color: u32,
    pub r#type: u8,
}

impl FromStr for Danmaku {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| ())
    }
}

/// Payload itself.
///
/// Used between the webSocket client and websocket actor model.
#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Danmaku(Danmaku),
    PlainText(String),
}

// pub struct Message(pub String);

/// Message that sent from `ChatServer` to `WsChatSession`,
/// usually used for broadcast.
#[derive(Message, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
#[rtype(result = "()")]
pub enum ServerMessage {
    Danmaku(Danmaku),
    /// Total Number of current room.
    StatisticInfo(usize),
}

/// Message that be sent from `WsChatSession` to `ChatServer`,
/// then it will be broadcast.
#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct DanmakuMessage {
    /// id of user that sent this message
    pub id: usize,
    pub danmaku: Danmaku,
    /// room name.
    pub room: String,
}

/// Connect Signal
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    /// the address of `WsChatSession`.
    pub addr: Recipient<ServerMessage>,
    /// the room name.
    pub room: String,
}

/// Disconnect Signal
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub room: String,
}

impl FromStr for ClientMessage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| ())
    }
}

impl From<DanmakuMessage> for ServerMessage {
    fn from(d: DanmakuMessage) -> Self {
        ServerMessage::Danmaku(d.danmaku)
    }
}

pub mod server;
pub mod session;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_client_message_parser() {
        let s = r#"{"type": "Danmaku", "data": {"type": 0, "text": "asdf", "color": 5592405}}"#;
        assert_eq!(true, serde_json::from_str::<ClientMessage>(s).is_ok());

        let s = r#"{"type": "PlainText", "data": "hahaha"}"#;
        assert_eq!(true, serde_json::from_str::<ClientMessage>(s).is_ok());

        let s = r#"{"type": "PlainText", "data":  {"text": "asdf"}}"#;
        assert_eq!(true, serde_json::from_str::<ClientMessage>(s).is_err());
    }

    #[test]
    fn test_server_message_parser() {}
}
