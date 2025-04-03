use crate::AppState;
use crate::models::task::{Duration, Importance, Priority, Status, Task};
use axum::extract::State;
use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, Redirect},
};
use minijinja::context;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Input {
    description: String,
    priority: Priority,
    importance: Importance,
    duration: Duration,
    status: Status,
}

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

pub async fn create(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("task.create").unwrap();

    let rendered = template
        .render(context! {
            all_priority => Priority::all(),
            all_importance => Importance::all(),
            all_duration => Duration::all(),
            all_status => Status::all(),
        })
        .unwrap();
    Ok(Html(rendered))
}

pub async fn insert(State(state): State<Arc<AppState>>, Form(input): Form<Input>) -> Redirect {
    let conn = state.db.lock().unwrap();

    let task = Task {
        description: input.description,
        priority: input.priority,
        importance: input.importance,
        duration:input.duration,
        status:input.status,
        ..Task::default()
    };
    let _ = task.insert(&conn).map_err(|err| {
        eprintln!("Erreur sql: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });
    Redirect::to("/task")
}
