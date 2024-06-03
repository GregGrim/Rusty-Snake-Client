use futures_util::{StreamExt, SinkExt};
use tokio::sync::watch;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::{models::{GameData, PlayerAction}, SharedGameData, SharedPlayerID};


pub async fn run_ws_client(
    game_data: SharedGameData,
    mut player_action: watch::Receiver<PlayerAction>,
    player_id: SharedPlayerID
) {
    let url = "ws://127.0.0.1:3000";
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let notify_connected = serde_json::to_string(&PlayerAction::PlayerConnected).unwrap();
    ws_stream.send(Message::Text(notify_connected)).await.unwrap();

    loop {
        tokio::select! {
            _ = player_action.changed() => {
                let new_action = player_action.borrow().clone();
                let serialized_data = serde_json::to_string(&new_action).unwrap();
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
                    } else if let Ok(received_data) = serde_json::from_str::<String>(text) {
                        let mut player_id = player_id.lock().await;
                        *player_id = received_data;
                        // println!("Received player ID: {:?}", player_id);
                    }
                }
            }
        }
    }
}