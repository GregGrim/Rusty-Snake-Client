use rand::Rng;
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
    score: i32
}

impl PlayerData {
    pub fn new() -> PlayerData {
        let player = PlayerData {
            player_id: uuid::Uuid::new_v4().to_string(),
            snake: Snake::new(Direction::Right, vec![
                Point{x: 0, y: 0},
                Point{x: 1, y: 0},
                Point{x: 2, y: 0}
                ]),
            score: 0
        };
        player
    }
    pub fn move_snake(&mut self) {
        self.snake.move_snake()
    }
    pub fn change_direction(&mut self, new_direction: Direction) {
        self.snake.change_direction(new_direction);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {x: self.x ,y: self.y}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
} 

impl Direction {
    pub fn to_coordinates(&self) -> (i32, i32) {
        match *self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    pub fn random() -> Direction{
        let mut rng = rand::thread_rng();
        let num: u32 = rng.gen_range(1..=4);
        match num {
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Right,
            4 => Direction::Left,
            _ => unreachable!(),
        }
    }
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
    body: Vec<Point>  
}
impl Snake {

    pub fn new(
        direction: Direction,
        body: Vec<Point>
    ) -> Snake {
       Snake {direction, body} 
    }

    pub fn move_snake(&mut self) {
        
        for i in (1..self.body.len()).rev() {

            self.body[i].x = self.body[i-1].x;
            self.body[i].y = self.body[i-1].y;
        }  
        self.body[0].x += self.direction.to_coordinates().0;
        self.body[0].y += self.direction.to_coordinates().1;
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction
    }

    fn snake_collision(& self) -> bool {
        let head = &self.body[0];
        for i in (1..self.body.len()) {
            if self.body[i].x == head.x && self.body[i].y == head.y {
                return true;
            }
        }
        false
    }
    fn wall_collision(& self, map_size: i32) -> bool {
        let head = &self.body[0];
        head.x < 0 || 
        head.y < 0 || 
        head.x>map_size || 
        head.y > map_size
    }

    pub fn check_collision(& self, map_size: i32) -> bool {
        self.snake_collision() || self.wall_collision(map_size)
    }

    // pub fn to_player_data(&self, player_id: String, score: i32) -> PlayerData {
    //     PlayerData {
    //         player_id,
    //         snake_position: self.body.clone(),
    //         score,
    //     }
    // }  
}