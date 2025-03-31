use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

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
enum Duree {
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
    description: String,
    priority: Priority,
    importance: Importance,
    duree: Duree,
    date_creation: Option<DateTime<Local>>,
    date_realisation: Option<DateTime<Local>>,
    date_echeance: Option<DateTime<Local>>,
    statut: Statut,
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
