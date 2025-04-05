use crate::AppState;
use crate::models::task::{Duration, Importance, Priority, Status, Task};
use axum::extract::{Form, Path, State};
use axum::{
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
    grouping: String,
}

// retourne toutes les tâches

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
            all_priority => Priority::all(),
            all_importance => Importance::all(),
            all_duration => Duration::all(),
            all_status => Status::all(),
        })
        .unwrap();
    Ok(Html(rendered))
}

// retourne le formulaire de création de tache

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

// ajoute une nouvelle tâche en base

pub async fn insert(State(state): State<Arc<AppState>>, Form(input): Form<Input>) -> Redirect {
    let conn = state.db.lock().unwrap();

    let task = Task {
        description: input.description,
        priority: input.priority,
        importance: input.importance,
        duration: input.duration,
        status: input.status,
        ..Task::default()
    };
    let _ = task.insert(&conn).map_err(|err| {
        eprintln!("Erreur sql: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });
    Redirect::to("/task")
}

// retourne le formulaire de mise à jour pour une tache donnée

pub async fn edit(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("task.edit").unwrap();

    let conn = state.db.lock().unwrap();

    let task = Task::get_by_id(id, &conn).map_err(|err| {
        eprintln!("Erreur sql: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let rendered = template
        .render(context! {
            all_priority => Priority::all(),
            all_importance => Importance::all(),
            all_duration => Duration::all(),
            all_status => Status::all(),
            task => task
        })
        .unwrap();
    Ok(Html(rendered))
}

// met à jour les données d'une tâche en base et renvoie sur index

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
    Form(input): Form<Input>,
) -> Redirect {
    let conn = state.db.lock().unwrap();
    let task = Task {
        description: input.description,
        priority: input.priority,
        importance: input.importance,
        duration: input.duration,
        status: input.status,
        grouping: input.grouping,
        ..Task::default()
    };

    let _ = task.update(id, &conn).map_err(|err| {
        eprintln!("Erreur sql: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });
    Redirect::to("/task")
}

// supprime un enregistrement en base et renvoie sur index

pub async fn delete(Path(id): Path<u32>, State(state): State<Arc<AppState>>) -> Redirect {
    let conn = state.db.lock().unwrap();

    let _ = Task::delete(id, &conn).map_err(|err| {
        eprintln!("Erreur sql: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });
    Redirect::to("/task")
}

// Termine une tâche

pub async fn complete(Path(id): Path<u32>, State(state): State<Arc<AppState>>) -> Redirect {
    let conn = state.db.lock().unwrap();

    match Task::get_by_id(id, &conn) {
        Ok(mut task) => {
            task.status = Status::Finished;
            let _ = task.update(id, &conn).map_err(|err| {
                eprintln!("Erreur sql: {:?}", err);
            });
        }
        Err(err) => {
            eprintln!("Erreur sql: {:?}", err);
        }
    }
    Redirect::to("/task")
}

// Démarre une tache

pub async fn start(Path(id): Path<u32>, State(state): State<Arc<AppState>>) -> Redirect {
    let conn = state.db.lock().unwrap();

    match Task::get_by_id(id, &conn) {
        Ok(mut task) => {
            task.status = Status::InProgress;
            let _ = task.update(id, &conn).map_err(|err| {
                eprintln!("Erreur sql: {:?}", err);
            });
        }
        Err(err) => {
            eprintln!("Erreur sql: {:?}", err);
        }
    }
    Redirect::to("/task")
}
