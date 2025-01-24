use std::{net::SocketAddr, ops::ControlFlow};

use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;



pub async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {

    let (mut sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });

}

fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Binary(d) => {
            for byte in d {
                println!("{byte}");
            }
        }
        Message::Text(..) => {}
        Message::Ping(..) => {}
        Message::Pong(..) => {}
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(">>> {} sent close with code {} and reason '{}'", who, cf.code, cf.reason);
            } else {
                println!(">>> {who} send close without CloseFrame");
            }
            return ControlFlow::Break(())
        }
    }
    ControlFlow::Continue(())
}
