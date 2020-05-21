use super::*;
use serde::Deserialize;
use serde_json;
use std::str::FromStr;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response cause a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Deserialize)]
#[allow(dead_code)]
struct Danmaku {
    #[serde(skip_deserializing)]
    user: String,
    text: String,
    color: u32,
    r#type: u8,
}

impl FromStr for Danmaku {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| ())
    }
}

impl Danmaku {
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
                println!("Websocket Client heartbeat failed, disconnecting");

                // notify ChatServer
                act.addr.do_send(messages::Disconnect {
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

    fn is_admin(&self) -> bool {
        match self.identity {
            Identity::Admin(_) =>true,
            _ => false
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
        self.addr
            .send(messages::Connect {
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
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // todo: monitor disconnect.
        // notify chat server
        self.addr.do_send(messages::Disconnect {
            id: self.id,
            room: self.room.to_owned(),
        });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<messages::Message> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: messages::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        println!("WEBSOCKET MESSAGE: {:?}", msg);

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) if self.is_login() => {
                let msg = text.trim().to_owned();

                println!("[{:?}]: {}", self.identity, msg);

                if msg.starts_with('/') & msg.contains(' ') {
                    // todo: 2. 获取统计信息（房间成员，房间弹幕） 这应该是 actor 的定时任务
                    // todo: 3. 获取某个成员统计信息（发弹幕数，登录时间，错误次数）
                } else {
                    // it's a danmaku format

                    // parse to check if it's a valid `Danmaku` format
                    if let Ok(danmaku) = msg.parse::<Danmaku>() {
                        match danmaku.valid() {
                            Ok(_) => {
                                // send message to chat server
                                self.addr.do_send(messages::ClientMessage {
                                    id: self.id,
                                    msg,
                                    room: self.room.to_owned(),
                                });
                            }
                            Err(e) => {
                                println!("Err: {}", e);
                            }
                        }
                    } else {
                        println!("parse err!");
                    };
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
            _ => (),
        }
    }
}
