use crate::AppState;
use crate::models::task::Task;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;

pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("task.index").unwrap();
    
    let conn = state.db.lock().unwrap();

    let tasks = Task::get_all(&conn).map_err(|err| {
        eprintln!("Erreur sql: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?; 

    let rendered = template
        .render(context! {
            title => "Todo liste",
            tasks => tasks,
        })
        .unwrap();
    Ok(Html(rendered))
}
