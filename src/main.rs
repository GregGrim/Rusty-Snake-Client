use std::sync::Arc;

use actix_web::{get, post, web::{self, to}, App, HttpResponse, HttpServer, Responder};
use engine::models::{GameData, PlayerData};
use tokio::sync::Mutex;

mod client;
mod message;
mod utils;
mod engine;

type SharedGameData = Arc<Mutex<GameData>>;
type SharedPlayerData = Arc<Mutex<PlayerData>>;

#[get("/snake")]
async fn get_data(game_data: web::Data<SharedGameData>) -> impl Responder{
    let game_data = game_data.lock().await;
    HttpResponse::Ok().json(&*game_data)
}

#[post("/snake")]
async fn send_data(player_data: web::Json<PlayerData>, shared_player_data: web::Data<SharedPlayerData>) -> impl Responder {
    let mut player_data_lock = shared_player_data.lock().await;
    *player_data_lock = player_data.into_inner();
    HttpResponse::Ok()
}

#[get("/start")]
async fn start_game(game_data: web::Data<SharedGameData>, player_data: web::Data<SharedPlayerData>) -> impl Responder {
    
    tokio::spawn(client::run_ws_client(Arc::clone(&game_data), Arc::clone(&player_data)));
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

    HttpServer::new(move || {
        App::new()
            .app_data(game_data.clone())
            .app_data(player_data.clone())
            .service(get_data)
            .service(send_data)
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
