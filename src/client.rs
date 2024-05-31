use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
pub async fn run() {
    let url = "wss://echo.websocket.events";

    println!("Hello, world!");

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect!");

    println!("Connected to Agent Network");

    let (mut write, mut read) = ws_stream.split();

    let msg = Message::Text("Privet".into());

    write.send(msg).await.expect("Failed to send msg");

    if let Some(message) = read.next().await{
        let message = message.expect("Failed to read the msg");
        println!("Received a messaage: {}", message)
    }
}
