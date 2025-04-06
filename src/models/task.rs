use crate::enum_with_strings;

use chrono::{NaiveDate, Local};
use rusqlite::{Connection, Result, params};
use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};

enum_with_strings!(Priority {
    ToBeDefined => "A définir",
    VeryUrgent => "Très urgent",
    Urgent => "Urgent",
    Normal => "Normal",
    NotUrgent => "Pas urgent",
});

enum_with_strings!(Importance {
    ToBeDefined => "A définir",
    VeryImportant => "Très important",
    Important => "Important",
    Normal => "Normal",
    NotImportant => "Pas important"
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
    Blocked => "Bloqué",
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub priority: Priority,
    pub importance: Importance,
    pub duration: Duration,
    pub creation_date: NaiveDate,
    pub completion_date: Option<NaiveDate>,
    pub start_date: Option<NaiveDate>,
    pub status: Status,
    pub grouping: String,
    pub scoring: u8
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
            completion_date: None,
            start_date: None,
            status: Status::ToDo,
            scoring:0,
            grouping:"".to_string()
        }
    }
}

impl Task {

    pub fn update_scoring(&mut self){
        self.scoring=0;

        self.scoring += match self.priority {
            Priority::ToBeDefined => 0,
            Priority::NotUrgent => 1,
            Priority::Normal => 2,
            Priority::Urgent => 3,
            Priority::VeryUrgent => 4,
        };

        self.scoring += match self.importance {
            Importance::ToBeDefined =>0,
            Importance::NotImportant => 1,
            Importance::Normal => 2,
            Importance::Important => 3,
            Importance::VeryImportant => 4
        };

        self.scoring += match self.duration {
            Duration::ToBeDefined => 0,
            Duration::VeryLong => 1,
            Duration::Long => 2,
            Duration::Normal => 3,
            Duration::Short => 4,
            Duration::VeryShort => 5
        };

        self.scoring *= match self.status {
            Status::Canceled =>0,
            Status::Finished =>1,
            Status::Blocked =>2,
            Status::ToDo => 3,
            Status::InProgress => 4
        }

    }

    // Ramène la liste des tâches
    pub fn get_all(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare("SELECT id, description, priority, importance, duration, creation_date, completion_date, start_date, status, grouping, scoring FROM tasks ORDER BY scoring desc")?;
        
        let tasks: Vec<Task> = stmt.query_map(params![], |row| {
            Ok(Task {
                id: row.get("id")?,
                description: row.get("description")?,
                priority: row.get("priority")?,
                importance: row.get("importance")?,
                duration: row.get("duration")?,
                creation_date: row.get("creation_date")?,
                completion_date: row.get("completion_date")?,
                start_date: row.get("start_date")?,
                status: row.get("status")?,
                grouping: row.get("grouping")?,
                scoring: row.get("scoring")?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(tasks)
    }

    // insert les données dans la base
    pub fn insert(&self, conn: &Connection) -> Result<usize>{
        conn.execute("INSERT INTO tasks (description, priority, importance, duration, creation_date, completion_date, start_date, status, grouping) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
        (&self.description, &self.priority, &self.importance, 
        &self.duration, &self.creation_date, &self.completion_date, 
        &self.start_date, &self.status, &self.grouping),)
    }

    // lit un id 
    pub fn get_by_id(id:u32,conn: &Connection) -> Result<Task>{
        conn.query_row(
            "SELECT id, description, priority, importance, duration, creation_date, completion_date, start_date, status, grouping, scoring FROM tasks WHERE id = ?1",
            params![id],
            |row| {Ok(Task {
                id: row.get("id")?,
                description: row.get("description")?,
                priority: row.get("priority")?,
                importance: row.get("importance")?,
                duration: row.get("duration")?,
                creation_date: row.get("creation_date")?,
                completion_date: row.get("completion_date")?,
                start_date: row.get("start_date")?,
                status: row.get("status")?,
                grouping: row.get("grouping")?,
                scoring:  row.get("scoring")?,
            })},
        )
    }

    // met à jour les données dans la base
    pub fn update(&mut self,id:u32, conn: &Connection) -> Result<usize>{
        self.update_scoring();
        conn.execute("UPDATE tasks SET description = ?1, priority = ?2, importance = ?3, duration = ?4, creation_date = ?5, completion_date = ?6, start_date = ?7, status = ?8, grouping = ?9, scoring = ?10 WHERE id = ?11;",
        (&self.description, &self.priority, &self.importance, 
        &self.duration, &self.creation_date, &self.completion_date, 
        &self.start_date, &self.status, &self.grouping,&self.scoring, id),)
    }

    pub fn delete(id:u32, conn: &Connection)-> Result<usize>{
        conn.execute("DELETE FROM tasks WHERE id=?1;",params![id],)
    }

    pub fn update_status(id:u32, status:Status,conn: &Connection)-> Result<usize>{
        let rqt = match status {
            Status::Finished => "UPDATE tasks SET status = ?1, completion_date = ?2 WHERE id=?3;",
            Status::InProgress => "UPDATE tasks SET status = ?1, start_date = ?2 WHERE id=?3;",
            _ => "UPDATE tasks SET status = ?1 WHERE id=?3;"
        };
        conn.execute(rqt,params![status,Local::now().date_naive(),id],)
    }
}
