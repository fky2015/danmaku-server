//! Define all the message used by actor models.

use crate::actor_models::PayloadType;
use crate::actor_models::WsChatSession;
use actix::*;
use std::str::FromStr;
use crate::actor_models::session::Danmaku;

/// Message that sent from `ChatServer` to `WsChatSession`,
/// usually used for broadcast.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message that be sent from `WsChatSession` to `ChatServer`,
/// then it will be broadcast.

#[derive(Message, Clone, Debug,)]
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
    pub addr: Recipient<DanmakuMessage>,
    /// the room name.
    pub room: String,
}

/// Connect Signal for Monitor
#[derive(Message)]
#[rtype(usize)]
pub struct MonitorConnect {
    /// the address of `WsChatSession`.
    pub addr: Addr<WsChatSession>,
}

/// Disconnect Signal
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub room: String,
}

/// Disconnect Signal
#[derive(Message)]
#[rtype(result = "()")]
pub struct MonitorDisconnect {
    pub id: usize,
}
