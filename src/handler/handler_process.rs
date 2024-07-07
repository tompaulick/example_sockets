use tokio::time::{sleep, Duration};
use crate::msg::msg_out::{MsgOut, ProcessUpdate};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use futures_util::SinkExt;

pub async fn process_this(ws: &mut WebSocketStream<TcpStream>) {
    let updates = [
        "complete gate 1",
        "complete gate 2",
        "complete gate 3"
    ];

    for update in updates.iter() {
        sleep(Duration::from_secs(1)).await;
        let msg_out = MsgOut::ProcessUpdate(ProcessUpdate {
            update: update.to_string(),
        });
        let msg_text = serde_json::to_string(&msg_out).expect("Failed to serialize");
        if let Err(e) = ws.send(Message::Text(msg_text)).await {
            eprintln!("Failed to send message: {}", e);
            break;
        }
    }
}
