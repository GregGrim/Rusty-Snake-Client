use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    players: Vec<PlayerData>,
    food: Point,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerData{
    player_id: String,
    snake_position: Vec<Point>,
    score: i32
}

impl PlayerData {
    pub fn new() -> PlayerData {
        let player = PlayerData {
            player_id: uuid::Uuid::new_v4().to_string(),
            snake_position: vec![
                Point{x: 0, y: 0},
                Point{x: 1, y: 0},
                Point{x: 2, y: 0}
                ],
            score: 0
        };
        player
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point{
    x: i32,
    y: i32
}