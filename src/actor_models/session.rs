//! Implement behaviour of `WsChatSession`.

use super::*;
use log::{info, warn};
use std::str::FromStr;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response cause a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

impl Danmaku {
    // TODO 合并 message processor 到这里
    #[allow(dead_code)]
    fn valid(&self) -> Result<(), &str> {
        // text length check
        if self.text.len() > 30 {
            return Err("text length larger than limit.");
        }

        Ok(())
    }
}

impl WsChatSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                warn!("Websocket Client heartbeat failed, disconnecting");

                // notify ChatServer
                act.addr.do_send(Disconnect {
                    id: act.id,
                    room: act.room.to_owned(),
                });

                // stop actor
                ctx.stop();

                // don'i try to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }

    fn is_login(&self) -> bool {
        self.identity != Identity::Anonymous
    }

    #[allow(dead_code)]
    fn is_admin(&self) -> bool {
        match self.identity {
            Identity::Admin(_) => true,
            _ => false,
        }
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // todo: monitor connect
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        info!("{:?} try to join in", self.identity);
        self.addr
            .send(Connect {
                addr: addr.recipient(),
                room: self.room.to_owned(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // somethings is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);

        let t = serde_json::to_string(&ServerMessage::from(self.identity.clone())).unwrap();
        ctx.text(t);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // todo: monitor disconnect.
        // notify chat server
        self.addr.do_send(Disconnect {
            id: self.id,
            room: self.room.to_owned(),
        });
        Running::Stop
    }
}

// impl Handler<DanmakuMessage> for WsChatSession {
//     type Result = ();
//     fn handle(&mut self, msg: DanmakuMessage, ctx: &mut Self::Context) {
//         // convert from DanmakuMessage to
//         println!(
//             "[{}] get message from server, send to peer WSSession.",
//             self.id
//         );
//         ctx.text(msg.danmaku.text);
//     }
// }

/// Handle messages from chat server, we simply send it to peer websocket.
impl Handler<ServerMessage> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        let serialized = serde_json::to_string(&msg).unwrap();
        ctx.text(serialized);
    }
}

/// Handle messages from peer websocket, we should apply all the check before we make decision.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        // TODO: log
        // println!("WEBSOCKET MESSAGE: {:?}", msg);

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            // Only `Admin` and `User` can send message to server.
            ws::Message::Text(text) if self.is_login() => {
                let msg = text.trim().to_owned();
                info!("WEBSOCKET MESSAGE: {:?}", msg);

                // 这里会进行相关处理。
                // 与 `ChatServer` 的分工不同，
                // 这里处理的依据是 `payload.data` 以外的部分。
                // 比如 `is_login`, 消息类型, 发送频率，是否在黑名单之类的。
                if let Ok(client_message) = msg.parse::<ClientMessage>() {
                    // we can get it's type.
                    match client_message {
                        // PayloadType::Danmaku => {
                        //     if let Ok(dplayer_danmaku) = payload.data.parse::<Danmaku>() {
                        //         // process
                        //         if let Ok(valid_danmaku) =
                        //             self.message_processor.process(dplayer_danmaku)
                        //         {
                        //             self.addr.do_send(DanmakuMessage {
                        //                 id: self.id,
                        //                 // 为什么要分开，因为这样后面就不需要再 parse 了
                        //                 danmaku: valid_danmaku,
                        //                 room: self.room.to_owned(),
                        //             })
                        //         } else {
                        //             // else not valid
                        //         }
                        //     } else {
                        //         // parse error
                        //     }
                        // }
                        // PayloadType::PlainDanmakuText => self.addr.do_send(DanmakuMessage {
                        //     id: self.id,
                        //     danmaku: Danmaku {
                        //         user: "".to_string(),
                        //         text: payload.data,
                        //         color: 0,
                        //         r#type: 0,
                        //     },
                        //     room: self.room.to_owned(),
                        // }),
                        ClientMessage::Danmaku(danmaku) => {
                            if let Ok(valid_danmaku) = self.message_processor.process(danmaku) {
                                self.addr.do_send(DanmakuMessage {
                                    id: self.id,
                                    // 为什么要分开，因为这样后面就不需要再 parse 了
                                    danmaku: valid_danmaku,
                                    room: self.room.to_owned(),
                                })
                            } else {
                                // else not valid
                            }
                        }
                        ClientMessage::PlainText(text) => {
                            if let Ok(valid_danmaku) = self
                                .message_processor
                                .process(Danmaku::from_str(&text).unwrap())
                            {
                                self.addr.do_send(DanmakuMessage {
                                    id: self.id,
                                    // 为什么要分开，因为这样后面就不需要再 parse 了
                                    danmaku: valid_danmaku,
                                    room: self.room.to_owned(),
                                })
                            } else {
                                // else not valid
                            }
                        }
                    }
                } else {
                    // throw a parse err!
                    // TODO: it may be a malicious behaviour.
                    warn!("parse err!");
                }

                // println!("[{:?}]: {}", self.identity, msg);
                //
                // if msg.starts_with('/') & msg.contains(' ') {
                //     // todo: 2. 获取统计信息（房间成员，房间弹幕） 这应该是 actor 的定时任务
                //     // todo: 3. 获取某个成员统计信息（发弹幕数，登录时间，错误次数）
            }
            // `Anonymous` will be rejected.
            ws::Message::Text(text) => {
                // TODO logger
                let msg = text.trim().to_owned();
                warn!(
                    "[{:?}]: message [{}] reject (due to not login)",
                    self.identity, msg
                );
            }
            ws::Message::Binary(_) => warn!("Unexpected binary"),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
