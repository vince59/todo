mod controllers;
mod models;
mod utils;
mod views;

use std::sync::{Arc, Mutex};
use axum::{Router, http::header, response::IntoResponse, routing::get};
use axum::extract::ws::{Message, WebSocket,WebSocketUpgrade};
use minijinja::Environment;
use rusqlite::Connection;
use futures::stream::StreamExt;

const BOOTSTRAP_CSS: &[u8] = include_bytes!("./static/css/bootstrap.min.css");
const BOOTSTRAP_JS: &[u8] = include_bytes!("./static/js/bootstrap.bundle.min.js");

struct AppState {
    env: Environment<'static>,
    db: Arc<Mutex<Connection>>,
}

async fn serve_bootstrap_css() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], BOOTSTRAP_CSS.to_vec())
}

// Fonction pour servir bootstrap.bundle.min.js
async fn serve_bootstrap_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript")],
        BOOTSTRAP_JS.to_vec(),
    )
}

// Endpoint qui upgrade en WebSocket
async fn handle_ws(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

// Gère la connexion WebSocket une fois établie
async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                println!("Reçu: {}", text);
                let _ = socket.send(Message::Text(format!("Echo: {}", text).into())).await;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let port;
    match utils::get_port_from_args() {
        Ok(p) => port = p,
        Err(err) => {
            eprintln!("{}", err);
            utils::print_usage();
            std::process::exit(1);
        }
    }

    let conn = Arc::new(Mutex::new(
        Connection::open(".\\todo.db").expect("❌ Erreur de connexion"),
    ));

    println!("Server starts on port : {port}");
    println!("Type http://localhost:{port} in your browser.");

    let mut env = Environment::new();
    views::template::add_template(&mut env);
    env.add_filter("format_date", utils::format_date);

    let app_state = Arc::new(AppState { env, db: conn });
    let app = Router::new()
        .route("/", get(controllers::home::controller_home))
        .route("/task", get(controllers::task::index))
        .route("/task/focus", get(controllers::task::focus))
        .route("/task/filter", get(controllers::task::filter))
        .route("/task/create", get(controllers::task::create).post(controllers::task::insert),)
        .route("/task/{id}/edit", get(controllers::task::edit).post(controllers::task::update),)
        .route("/task/{id}/delete", get(controllers::task::delete),)
        .route("/task/{id}/update_status", get(controllers::task::update_status),)
        .route("/test", get(controllers::task::test),)
        .route("/css/bootstrap.min.css", get(serve_bootstrap_css))
        .route("/js/bootstrap.bundle.min.js", get(serve_bootstrap_js))
        .route("/ws", get(handle_ws))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
