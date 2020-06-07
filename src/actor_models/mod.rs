//! A module stored actor models.
//!
//! The `actor_models` module contains all the things that actor needs.
//!
//! Two main actor model is [`ChatServer`] and [`WsChatSession`].

use std::collections::{HashMap, HashSet};

use crate::messages;
use actix_web_actors::ws;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::message_processor::MessageProcessor;
use actix::*;
use serde::Deserialize;
use std::str::FromStr;
use std::time::{Duration, Instant};

/// `ChatServer` handle all the connections, messages and disconnections.
/// There is only one ChatServer at a time.
pub struct ChatServer {
    pub sessions: HashMap<usize, Recipient<messages::DanmakuMessage>>,
    pub rooms: HashMap<String, HashSet<usize>>,
    pub rng: ThreadRng,
    // pub monitors: HashMap<usize, Context<WsChatSession>>,
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

/// Payload type.
#[derive(Deserialize, Debug)]
pub enum PayloadType {
    Danmaku,
    PlainDanmakuText,
}

/// Payload itself.
#[derive(Deserialize, Debug)]
struct Payload {
    r#type: PayloadType,
    data: String,
}

impl FromStr for Payload {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| ())
    }
}

pub mod server;

pub mod session;
