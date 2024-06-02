use std::time::Duration;

use models::{Direction, Point};
use tokio::time;

use crate::{FoodUpdatedState, SharedGameData, SharedPlayerData};

pub mod models;


pub async fn run_snake_engine(player_data: SharedPlayerData, game_data: SharedGameData, food_updated: FoodUpdatedState) {

    let mut interval = time::interval(Duration::from_millis(500));
    loop {
        interval.tick().await;
        let mut player_data = player_data.lock().await;
        player_data.move_snake();
        
        let mut game_data = game_data.lock().await;
        let food = game_data.get_food();
        if player_data.food_collision(food) {
            game_data.set_food();
            let mut food_updated = food_updated.lock().await;
            *food_updated = true;
        }
    }
}

pub async fn change_direction(player_data: SharedPlayerData, new_direction: &str) {
    let mut player_data = player_data.lock().await;
    player_data.change_direction(Direction::map(new_direction));
}