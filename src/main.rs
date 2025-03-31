mod models;
mod views;
mod controllers;
mod utils;

use axum::{Router, routing::get};
use minijinja::Environment;
use std::sync::Arc;

struct AppState {
    env: Environment<'static>,
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

    println!("Server starts on port : {port}");
    println!("Type http://localhost:{port} in your browser.");

    let mut env = Environment::new();
    views::template::add_template(&mut env);
    // pass env to handlers via state
    let app_state = Arc::new(AppState { env });
    // Ajout des routes
    let app = Router::new()
        .route("/", get(controllers::home::controller_home))
        .route("/task", get(controllers::task::index))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
