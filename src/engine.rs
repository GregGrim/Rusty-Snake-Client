use std::time::Duration;

use models::{Direction, Snake};
use tokio::time;

use crate::SharedPlayerData;

pub mod models;


pub async fn run_snake_engine(player_data: SharedPlayerData) {

    let mut interval = time::interval(Duration::from_millis(500));
    loop {
        interval.tick().await;
        let mut player_data = player_data.lock().await;
        player_data.move_snake();
    }
}

pub async fn change_direction(player_data: SharedPlayerData, new_direction: &str) {
    let mut player_data = player_data.lock().await;
    player_data.change_direction(Direction::map(new_direction));
}