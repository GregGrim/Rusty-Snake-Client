use std::time::Duration;
use futures_util::{StreamExt, SinkExt};
use tokio::time;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::{engine::models::{GameData, PlayerData}, SharedGameData, SharedPlayerData};


pub async fn run_ws_client(game_data: SharedGameData, player_data: SharedPlayerData) {
    let url = "ws://127.0.0.1:3000";
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let mut interval = time::interval(Duration::from_secs(1));

    *player_data.lock().await = PlayerData::new();

    loop {
        tokio::select! {
            _ = interval.tick() => {     
                let updated_data = &*player_data.lock().await;
                let serialized_data = serde_json::to_string(&updated_data).unwrap();
                ws_stream.send(Message::Text(serialized_data)).await.unwrap();
            }
            Some(msg) = ws_stream.next() => {
                let msg = msg.expect("Failed to read message");
                if msg.is_text() {
                    let text = msg.to_text().unwrap();
                    if let Ok(received_data) = serde_json::from_str::<GameData>(text) {
                        let mut game_data = game_data.lock().await;
                        *game_data = received_data;
                        // println!("Received game data: {:?}", game_data);
                    }
                }
            }
        }
    }
}