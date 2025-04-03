use crate::enum_with_strings;
use crate::utils::ToStr;

use chrono::{DateTime, Local};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

enum_with_strings!(Priority {
    ToBeDefined => "A définir",
    VeryUrgent => "Très urgent",
    Urgent => "Urgent",
    Normal => "Normal",
    NotUrgent => "Pas urgent",
    NotAtAllUrgent => "Pas du tout urgent",
});

enum_with_strings!(Importance {
    ToBeDefined => "A définir",
    VeryImportant => "Très important",
    Important => "Important",
    Normal => "Normal",
    NotImportant => "Pas important",
    NotAtAllImportant => "Pas du tout important",
});

enum_with_strings!(Duration {
    ToBeDefined => "A définir",
    VeryLong => "Très long",
    Long => "Long",
    Normal => "Normal",
    Short => "Rapide",
    VeryShort => "Très court",
});

enum_with_strings!(Status {
    ToDo => "A faire",
    InProgress => "En cours",
    Finished => "Fini",
    Canceled => "Annulé",
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<u32>,
    pub description: String,
    pub priority: Priority,
    pub importance: Importance,
    pub duration: Duration,
    pub creation_date: Option<DateTime<Local>>,
    pub completion_date: Option<DateTime<Local>>,
    pub due_date: Option<DateTime<Local>>,
    pub status: Status,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: None,
            description: "Description de la tâche".to_string(),
            priority: Priority::ToBeDefined,
            importance: Importance::ToBeDefined,
            duration: Duration::ToBeDefined,
            creation_date: Some(Local::now()),
            completion_date: None,
            due_date: None,
            status: Status::ToDo,
        }
    }
}

impl Task {
    pub fn insert(&self, conn: &Connection) -> Result<usize>{
        conn.execute("INSERT INTO tasks (description, priority, importance, duration, creation_date, completion_date, due_date, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
        (&self.description, &self.priority.to_str(), &self.importance.to_str(), 
        &self.duration.to_str(), &self.creation_date.to_str(), &self.completion_date.to_str(), 
        &self.due_date.to_str(), &self.status.to_str()),)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Task>> {
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
