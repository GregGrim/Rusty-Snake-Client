use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub player_id: u32,
    pub position: (i32, i32),
    pub direction: String,
    pub score: u32,
}

impl GameData {
    pub fn new() -> GameData {
        let game_data = GameData {
            player_id: 1,
            position: (0, 0),
            direction: "up".to_string(),
            score: 0,
        };
        game_data
    }
}