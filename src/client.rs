use std::sync::Arc;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message as WsMessage;

use crate::message::Message;
use crate::utils::current_timestamp;

// TODO move to db
struct ClientInfo {
    client_id: Option<String>,
}

impl ClientInfo {
    fn new() -> Self {
        Self { client_id: None }
    }

    fn update_client_id(&mut self, client_id: String) {
        self.client_id = Some(client_id);
    }
}

async fn handle_message(message: WsMessage) {
        if message.is_text() {
            let text = message.to_text().unwrap();
            if let Ok(received_msg) = serde_json::from_str::<Message>(text) {
                match received_msg {
                    Message::Chat { sender, recipient, content, timestamp } => {
                        println!("Received a message from {}: {} at {}", sender, content, timestamp);
                    },
                    _ => {}
                }
            }
        }
}

async fn receive_loop(mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {
    loop {
        if let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    handle_message(msg).await;
                }
                Err(err) => {
                    eprintln!("WebSocket error: {}", err);
                    break;
                }
            }
        }
    }
}

async fn send_loop(write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>, client_info: ClientInfo) {
    loop {
        send_message(&write, &client_info).await;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn send_message(write: &Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>,client_info: &ClientInfo) {
    let mut write = write.lock().await;

    let sender_id:String = client_info.client_id.clone().unwrap_or("Unknown".to_string());

    let chat_message = Message::Chat {
        sender: sender_id.clone(),
        recipient: sender_id, // For testing, send to self
        content: "Hello, self!".to_string(),
        timestamp: current_timestamp(),
    };

    let msg_text = serde_json::to_string(&chat_message).unwrap();
    let msg = WsMessage::Text(msg_text.clone());

    if let Err(err) = write.send(msg).await {
        eprintln!("Failed to send message: {:?}", err);
    } else {
        println!("Sended msg {}", msg_text);
    }
}


#[tokio::main]
pub async fn run() {
    let url = "ws://127.0.0.1:3000";

    println!("Connecting to {}", url);

    

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect!");

    let (write,mut read) = ws_stream.split();
    let write = Arc::new(Mutex::new(write));

    let mut client_info = ClientInfo::new();

    // receiving connected message with ID
    if let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();
                    if let Ok(received_msg) = serde_json::from_str::<Message>(text) {
                        match received_msg {
                            Message::UserConnected {client_id, ..} => {
                                println!("Connected to WebSocket server. Given client ID: {}", client_id);
                                client_info.update_client_id(client_id);
                            },
                            _ => {}
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("WebSocket error: {}", err);
            }
        }
    }
    
    let receive_task = tokio::spawn(receive_loop(read));
    let send_task = tokio::spawn(send_loop(write.clone(), client_info));


    receive_task.await.expect("Receive task failed");
    send_task.await.expect("Send task failed");

    // write.close().await.expect("Failed to close connection"); 

}
