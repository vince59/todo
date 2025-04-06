use crate::AppState;
use crate::models::task::{Duration, Importance, Priority, Status, Task};
use crate::utils::parse_optional_date;
use axum::extract::{Form, Path, Query, State};
use axum::{
    http::StatusCode,
    response::{Html, Redirect},
};
use chrono::NaiveDate;
use minijinja::context;
use serde::Deserialize;
use std::sync::Arc;

pub trait ToTask {
    fn to_task(&self) -> Task;
}

// Structure pour récupérer les données du formulaire html de création de tâche

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct CreateTaskForm {
    description: String,
    priority: Priority,
    importance: Importance,
    duration: Duration,
    status: Status,
    grouping: String,
}

impl ToTask for CreateTaskForm {
    fn to_task(&self) -> Task {
        Task {
            description: self.description.clone(),
            priority: self.priority,
            importance: self.importance,
            duration: self.duration,
            status: self.status,
            grouping: self.grouping.clone(),
            ..Task::default()
        }
    }
}

// Structure pour récupérer les données du formulaire html d'édition de tâche

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct EditTaskForm {
    description: String,
    priority: Priority,
    importance: Importance,
    duration: Duration,
    status: Status,
    grouping: String,
    creation_date: NaiveDate,
    completion_date: String,
    start_date: String,
}

impl ToTask for EditTaskForm {
    fn to_task(&self) -> Task {
        Task {
            description: self.description.clone(),
            priority: self.priority,
            importance: self.importance,
            duration: self.duration,
            status: self.status,
            grouping: self.grouping.clone(),
            completion_date: parse_optional_date(&self.completion_date).unwrap(),
            start_date: parse_optional_date(&self.start_date).unwrap(),
            creation_date: self.creation_date,
            ..Task::default()
        }
    }
}

#[derive(Deserialize)]
pub struct StatusParam {
    status: Status,
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

pub async fn insert(
    State(state): State<Arc<AppState>>,
    Form(input): Form<CreateTaskForm>,
) -> Redirect {
    let conn = state.db.lock().unwrap();

    let _ = input.to_task().insert(&conn).map_err(|err| {
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
    Form(input): Form<EditTaskForm>,
) -> Redirect {
    let conn = state.db.lock().unwrap();
    let _ = input.to_task().update(id, &conn).map_err(|err| {
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

// Met à jour le statut de la tâche

pub async fn update_status(
    Path(id): Path<u32>,
    Query(param): Query<StatusParam>,
    State(state): State<Arc<AppState>>,
) -> Redirect {
    let conn = state.db.lock().unwrap();

    let _ = Task::update_status(id, param.status, &conn).map_err(|err| {
        eprintln!("Erreur sql: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    Redirect::to("/task")
}
