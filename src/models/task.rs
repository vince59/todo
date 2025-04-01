use chrono::{DateTime, Local};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Priority {
    ToBeDefined,
    VeryUrgent,
    Urgent,
    Normal,
    NotUrgent,
    NotAtAllUrgent,
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::ToBeDefined => "À définir".to_string(),
            Priority::VeryUrgent => "Très urgent".to_string(),
            Priority::Urgent => "Urgent".to_string(),
            Priority::Normal => "Normal".to_string(),
            Priority::NotUrgent => "Pas urgent".to_string(),
            Priority::NotAtAllUrgent => "Pas du tout urgent".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Importance {
    ToBeDefined,
    VeryImportant,
    Important,
    Normal,
    NotImportant,
    NotAtAllImportant,
}

impl ToString for Importance {
    fn to_string(&self) -> String {
        match self {
            Importance::ToBeDefined => "À définir".to_string(),
            Importance::VeryImportant => "Très important".to_string(),
            Importance::Important => "Important".to_string(),
            Importance::Normal => "Normal".to_string(),
            Importance::NotImportant => "Pas important".to_string(),
            Importance::NotAtAllImportant => "Pas du tout important".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Duration {
    ToBeDefined,
    VeryLong,
    Long,
    Normal,
    Short,
    VeryShort,
}

impl ToString for Duration {
    fn to_string(&self) -> String {
        match self {
            Duration::ToBeDefined => "À définir".to_string(),
            Duration::VeryLong => "Très long".to_string(),
            Duration::Long => "Long".to_string(),
            Duration::Normal => "Normal".to_string(),
            Duration::Short => "Rapide".to_string(),
            Duration::VeryShort => "Très court".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Finished,
    Canceled,
}

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
    pub fn create(&self, conn: &Connection) {
        let mut stmt = conn.execute("INSERT INTO tasks (description, priority, importance, duration, creation_date, completion_date, due_date, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 8?)",
        [&self.description, &self.priority.to_string(), &self.importance, &self.duration, &self.creation_date, &self.completion_date, &self.due_date, &self.status])?;
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
