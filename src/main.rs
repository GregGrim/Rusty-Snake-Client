use std::sync::Arc;

use actix_web::{get, post, web::{self}, App, HttpResponse, HttpServer, Responder};
use models::{Direction, GameData, PlayerAction};
use serde::Deserialize;
use tokio::sync::{watch, Mutex};

mod models;
mod client;
mod message;
mod utils;

type SharedGameData = Arc<Mutex<GameData>>;
type SharedPlayerID = Arc<Mutex<String>>;

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
    direction_param: web::Json<ChangeDirectionRequest>,
    player_action_tx: web::Data<watch::Sender<PlayerAction>>,
    player_id: web::Data<SharedPlayerID>
) -> impl Responder {
    let player_id = &*player_id.lock().await;
    let direction = Direction::map(direction_param.direction.as_str());
    player_action_tx.send(PlayerAction::PlayerChangedDirection(player_id.clone(), direction)).unwrap();
    HttpResponse::Ok()
}

#[get("/start")]
async fn start_game(
    player_action_tx: web::Data<watch::Sender<PlayerAction>>,
    player_id: web::Data<SharedPlayerID>
) -> impl Responder {
    let player_id = player_id.lock().await.clone();
    player_action_tx.send(PlayerAction::PlayerStartedGame(player_id.clone())).unwrap();
    HttpResponse::Ok().json(player_id)
}

#[get("/")]
async fn create_app(
    game_data: web::Data<SharedGameData>,
    player_action_tx: web::Data<watch::Sender<PlayerAction>>,
    player_id: web::Data<SharedPlayerID>
) -> impl Responder{
    // show starting page of the game where button start will be 
    tokio::spawn(client::run_ws_client(
        Arc::clone(&game_data),
        player_action_tx.subscribe(),
        Arc::clone(&player_id)
    ));
    let html = include_str!("web/game.html");

    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let game_data = web::Data::new(Arc::new(Mutex::new(GameData::new())));
    let (player_action_tx, _) = watch::channel(PlayerAction::PlayerConnected);
    let player_action_tx = web::Data::new(player_action_tx);
    let player_id = web::Data::new(Arc::new(Mutex::new(String::new())));

    HttpServer::new(move || {
        App::new()
            .app_data(game_data.clone())
            .app_data(player_id.clone())
            .app_data(player_action_tx.clone())
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
