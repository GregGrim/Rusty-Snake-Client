use std::time::Duration;

use models::Direction;
use tokio::time;

use crate::{FoodUpdatedState, GameOverState, SharedGameData, SharedPlayerData};

pub mod models;


pub async fn run_snake_engine(
    player_data: SharedPlayerData, 
    game_data: SharedGameData,
    food_updated: FoodUpdatedState,
    game_over: GameOverState
) {

    let mut interval = time::interval(Duration::from_millis(200));


    loop {
        interval.tick().await;

        {
            let mut player_data = player_data.lock().await;
            player_data.move_snake();
        }

        let mut game_data = game_data.lock().await;

        {

            let collision_detected = {
                let mut player_data = player_data.lock().await;
                game_data.check_players_collision(&player_data) || player_data.check_collision()
            };

            if collision_detected {
                println!("Collision detected. Game over.");
                let mut game_over = game_over.lock().await;
                *game_over = true;
                break;
            }

            let food_collision_detected = {
                let mut player_data = player_data.lock().await;
                player_data.food_collision(game_data.get_food())
            };

            if food_collision_detected {
                println!("Food collision detected. Updating food.");
                game_data.set_food();
                let mut food_updated = food_updated.lock().await;
                *food_updated = true;
            }
        }
    }
}

pub async fn change_direction(player_data: SharedPlayerData, new_direction: &str) {
    let mut player_data = player_data.lock().await;
    player_data.change_direction(Direction::map(new_direction));
}