use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{StreamExt, SinkExt};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;


async fn handle_incoming_messages(mut reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {
    while let Some(msg) = reader.next().await {
        let msg = msg.expect("Failed to read message!");
        println!("{}", msg.to_text().unwrap());
    }
}

async fn read_and_send_messages(mut writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>) {
    let mut reader = BufReader::new(io::stdin()).lines();
    while let Some(line) = reader.next_line().await.expect("Failed to create message") {
        if !line.trim().is_empty() {
            writer.send(WsMessage::Text(line)).await.expect("Failed to send new message");
        }
    }
}


#[tokio::main]
pub async fn run() {
    let url = "ws://127.0.0.1:3000";

    let (stream, _) = connect_async(url).await.expect("Failed to connect!");

    println!("Connected to the server!");

    let (writer, reader) = stream.split();

    let read_handle = tokio::spawn(handle_incoming_messages(reader));
    let write_handle = tokio::spawn(read_and_send_messages(writer));

    let _ = tokio::join!(read_handle, write_handle);
    
}