use std::collections::{HashMap, HashSet};

use crate::messages;
use actix_web_actors::ws;
use rand::rngs::ThreadRng;
use rand::Rng;

use actix::*;
use std::time::{Duration, Instant};
/// `ChatServer` handle all the connections, messages and disconnections.
/// There is only one ChatServer at a time.
pub struct ChatServer {
    pub sessions: HashMap<usize, Recipient<messages::Message>>,
    pub rooms: HashMap<String, HashSet<usize>>,
    pub rng: ThreadRng,
    // pub monitors: HashMap<usize, Context<WsChatSession>>,
}

#[derive(PartialEq, Debug)]
pub enum Identity {
    Anonymous,
    User(String),
    Admin(String),
}

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
}

pub mod server;

pub mod session;
