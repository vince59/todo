mod controllers;
mod models;
mod utils;
mod views;

use std::sync::{Arc, Mutex};
use axum::{Router, http::header, response::IntoResponse, routing::get};
use minijinja::Environment;
use rusqlite::Connection;

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
        Connection::open("C:\\rust\\todo\\todo.db").expect("❌ Erreur de connexion"),
    ));

    println!("Server starts on port : {port}");
    println!("Type http://localhost:{port} in your browser.");

    let mut env = Environment::new();
    views::template::add_template(&mut env);

    let app_state = Arc::new(AppState { env, db: conn });
    let app = Router::new()
        .route("/", get(controllers::home::controller_home))
        .route("/task", get(controllers::task::index))
        .route("/css/bootstrap.min.css", get(serve_bootstrap_css))
        .route("/js/bootstrap.bundle.min.js", get(serve_bootstrap_js))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
