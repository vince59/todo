use crate::AppState;
use crate::models::task::Task;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;

pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("task.index").unwrap();
    //let tasks = vec![Task::default();5];

    let conn = Task::connect().map_err(|err| {
        println!("Erreur de connexion à SQLite: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?; // Connexion à SQLite
    let tasks = Task::get_all_tasks(&conn).map_err(|err| {
        println!("Erreur de connexion à SQLite: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?; // Récupérer les enregistrements

    /*  for task in tasks {
        println!("Description: {}", task.description);
    }*/

    let rendered = template
        .render(context! {
            title => "Todo liste",
            tasks => tasks,
        })
        .unwrap();
    Ok(Html(rendered))
}
