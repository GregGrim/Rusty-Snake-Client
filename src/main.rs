use std::sync::Arc;

use actix_web::{get, post, web::{self}, App, HttpResponse, HttpServer, Responder};
use engine::models::{GameData, PlayerData};
use serde::Deserialize;
use tokio::sync::Mutex;

mod client;
mod message;
mod utils;
mod engine;

type SharedGameData = Arc<Mutex<GameData>>;
type SharedPlayerData = Arc<Mutex<PlayerData>>;
type FoodUpdatedState = Arc<Mutex<bool>>;
type GameOverState = Arc<Mutex<bool>>;

#[derive(Deserialize)]
struct ChangeDirectionRequest {
    direction: String,
}

#[get("/snake")]
async fn get_data(game_data: web::Data<SharedGameData>) -> impl Responder{
    let game_data = game_data.lock().await;
    HttpResponse::Ok().json(&*game_data)
}

#[post("/change_direction")]
async fn change_direction(
    player_data: web::Data<SharedPlayerData>, 
    direction_param: web::Json<ChangeDirectionRequest>
) -> impl Responder {
    engine::change_direction(Arc::clone(&player_data), direction_param.direction.as_str()).await;
    HttpResponse::Ok()
}

#[get("/start")]
async fn start_game(
    game_data: web::Data<SharedGameData>, 
    player_data: web::Data<SharedPlayerData>,
    food_updated: web::Data<FoodUpdatedState>,
    game_over: web::Data::<GameOverState>
) -> impl Responder {

    *food_updated.lock().await = false;
    *game_over.lock().await = false;
    
    tokio::spawn(engine::run_snake_engine(
        Arc::clone(&player_data),
        Arc::clone(&game_data),
        Arc::clone(&food_updated),
        Arc::clone(&game_over)
    ));

    tokio::spawn(client::run_ws_client(
        Arc::clone(&game_data), 
        Arc::clone(&player_data), 
        Arc::clone(&food_updated),
        Arc::clone(&game_over)
    ));
    HttpResponse::Ok().body("Game started")
}

#[get("/")]
async fn create_app() -> impl Responder{
    // show starting page of the game where button start will be 
    let html = include_str!("web/game.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let player_data = web::Data::new(Arc::new(Mutex::new(PlayerData::new())));

    let game_data = web::Data::new(Arc::new(Mutex::new(GameData::new())));

    let food_updated = web::Data::new(Arc::new(Mutex::new(false)));

    let game_over = web::Data::new(Arc::new(Mutex::new(false)));

    HttpServer::new(move || {
        App::new()
            .app_data(game_data.clone())
            .app_data(player_data.clone())
            .app_data(food_updated.clone())
            .app_data(game_over.clone())
            .service(get_data)
            .service(change_direction)
            .service(start_game)
            .service(create_app)
            .route(
                "script.js",
                web::get().to(|| async {
                     HttpResponse::Ok().content_type("application/javascript").body(include_str!("web/script.js")) 
                })
            )
            .route(
                "styles.css", web::get().to(|| async {
                     HttpResponse::Ok().content_type("text/css").body(include_str!("web/styles.css")) 
                })
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
