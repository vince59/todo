use crate::enum_with_strings;

use chrono::{NaiveDate, Local};
use rusqlite::{Connection, Result};
use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, ValueRef};
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
    pub id: u32,
    pub description: String,
    pub priority: Priority,
    pub importance: Importance,
    pub duration: Duration,
    pub creation_date: NaiveDate,
    pub completion_date: NaiveDate,
    pub due_date: NaiveDate,
    pub status: Status,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            description: "Description de la tâche".to_string(),
            priority: Priority::ToBeDefined,
            importance: Importance::ToBeDefined,
            duration: Duration::ToBeDefined,
            creation_date: Local::now().date_naive(),
            completion_date: Local::now().date_naive(),
            due_date: Local::now().date_naive(),
            status: Status::ToDo,
        }
    }
}

impl Task {
    pub fn insert(&self, conn: &Connection) -> Result<usize>{
        conn.execute("INSERT INTO tasks (description, priority, importance, duration, creation_date, completion_date, due_date, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
        (&self.description, &self.priority, &self.importance, 
        &self.duration, &self.creation_date, &self.completion_date, 
        &self.due_date, &self.status),)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare("SELECT id,description, priority, importance, duration, creation_date, completion_date, due_date, status FROM tasks")?;
        let tasks_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                priority: row.get(2)?,
                importance: row.get(3)?,
                duration: row.get(4)?,
                creation_date: row.get(5)?,
                completion_date: row.get(6)?,
                due_date: row.get(7)?,
                status: row.get(8)?
            })
        })?;

        let mut tasks = Vec::new();
        for task in tasks_iter {
            tasks.push(task?);
        }

        Ok(tasks)
    }
}
