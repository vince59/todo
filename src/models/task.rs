use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Priority {
    ADéfinir,
    TresUrgent,
    Urgent,
    Normal,
    PasUrgent,
    PasDuToutUrgent,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Importance {
    ADéfinir,
    TrèsImportant,
    Important,
    Normal,
    PasImportant,
    PasDuToutImportant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Duree {
    ADéfinir,
    TrèsLongue,
    Longue,
    Normal,
    Courte,
    TrèsCourte,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statut {
    AFaire,
    EnCours,
    Terminé,
    Annulé,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub priority: Priority,
    pub importance: Importance,
    pub duree: Duree,
    pub date_creation: Option<DateTime<Local>>,
    pub date_realisation: Option<DateTime<Local>>,
    pub date_echeance: Option<DateTime<Local>>,
    pub statut: Statut,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            description: "Description de la tâche".to_string(),
            priority: Priority::ADéfinir,
            importance: Importance::ADéfinir,
            duree: Duree::ADéfinir,
            date_creation: Some(Local::now()),
            date_realisation: None,
            date_echeance: None,
            statut: Statut::AFaire,
        }
    }
}

impl Task {
pub fn connect() -> Result<Connection> {
    Connection::open("C:\\rust\\todo\\todo.db") // Se connecte à la base SQLite
}

pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT description FROM tasks")?;
    let tasks_iter = stmt.query_map([], |row| {
        Ok(Task {
            description: row.get(0)?, 
            ..Task::default()
        })
    })?;

    let mut tasks = Vec::new();
    for task in tasks_iter {
        tasks.push(task?);
    }

    Ok(tasks)
}
}