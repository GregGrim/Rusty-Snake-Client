use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    players: Vec<PlayerData>,
    food: Point,
}

impl GameData {
    pub fn new() -> GameData{
        let game_data =  GameData {
            players: vec![],
            food: Point{x:10, y:10}
        };
        game_data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerData{
    player_id: String,
    snake: Snake,
    score: i32,
    game_over: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl Direction {
    pub fn map(s: &str) -> Direction{
        match s {
            "Up" => Direction::Up,
            "Down" => Direction::Down,
            "Left" => Direction::Left,
            "Right" => Direction::Right,
            _ => unreachable!()
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Snake {
    direction: Direction,
    body: Vec<Point>,
    has_eaten: bool  
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlayerAction {
    PlayerConnected,
    PlayerStartedGame(String),
    PlayerChangedDirection(String, Direction)
}