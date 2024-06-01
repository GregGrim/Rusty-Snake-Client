use std::time::Duration;
use futures_util::{StreamExt, SinkExt};
use tokio::time;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::engine::models::GameData;


#[tokio::main]
pub async fn run() {
    let url = "ws://127.0.0.1:3000";
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let game_data = GameData::new();

    let serialized_data = serde_json::to_string(&game_data).unwrap();
    ws_stream.send(Message::Text(serialized_data)).await.unwrap();

    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Simulate sending updated game data every second
                let updated_data = GameData {
                    player_id: game_data.player_id,
                    position: (game_data.position.0 + 1, game_data.position.1),
                    direction: game_data.direction.clone(),
                    score: game_data.score + 1,
                };

                let serialized_data = serde_json::to_string(&updated_data).unwrap();
                ws_stream.send(Message::Text(serialized_data)).await.unwrap();
            }
            Some(msg) = ws_stream.next() => {
                let msg = msg.expect("Failed to read message");
                if msg.is_text() {
                    let text = msg.to_text().unwrap();
                    if let Ok(received_data) = serde_json::from_str::<GameData>(text) {
                        println!("Received game data: {:?}", received_data);
                    }
                }
            }
        }
    }
}