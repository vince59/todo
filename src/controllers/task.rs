use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;
use crate::models::task::Task;
use crate::AppState;

pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("task.index").unwrap();
    let tasks = vec![Task::default();5];
    let rendered = template
        .render(context! {
            title => "Todo liste",
            tasks => tasks,
        })
        .unwrap();
    Ok(Html(rendered))
}
