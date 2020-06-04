//! Define all the message used by actor models.



use actix::*;
use crate::actor_models::WsChatSession;
use crate::actor_models::PayloadType;

/// Message that sent from `ChatServer` to `WsChatSession`,
/// usually used for broadcast.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message that be sent from `WsChatSession` to `ChatServer`,
/// then it will be broadcast.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// id of user that sent this ssage
    pub id: usize,
    pub r#type: PayloadType,
    /// message it self
    pub msg: String,
    /// room name.
    pub room: String,
}

/// Connect Signal
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    /// the address of `WsChatSession`.
    pub addr: Recipient<Message>,
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
#[rtype(result= "()")]
pub struct MonitorDisconnect {
    pub id: usize,
}