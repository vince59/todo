use crate::AppState;
use crate::models::task::{Task,Priority};
use std::sync::Arc;
use axum::extract::State;
use axum::{extract::Form,response::{Html,Redirect},http::StatusCode};
use serde::Deserialize;
use minijinja::context;


#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Input {
    description: String,
    priority: Priority
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
            priorities => Priority::all()
        })
        .unwrap();
    Ok(Html(rendered))
}

pub async fn insert( State(state): State<Arc<AppState>>, Form(input): Form<Input>,) -> Redirect  {
    dbg!(&input);
    let conn = state.db.lock().unwrap();

    let task = Task { description:input.description, priority:input.priority, ..Task::default()};
    let _ = task.insert(&conn).map_err(|err| {
        eprintln!("Erreur sql: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    }); 
    Redirect::to("/task")
}
