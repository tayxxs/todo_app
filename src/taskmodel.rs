use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
enum ItemStatus {
    Todo,
    Done,
    Shelved,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    name: String,
    status: ItemStatus,
    create_time: DateTime<Utc>,
    complete_time: DateTime<Utc>,
    shelve_time: DateTime<Utc>,
    delete_time: DateTime<Utc>,
}

impl Task {
    fn new(id: u32, name: String, status: ItemStatus, create_time: DateTime<Utc>, complete_time: DateTime<Utc>, shelve_time: DateTime<Utc>, delete_time: DateTime<Utc>) -> Self {
        Task { id, name, status, create_time, complete_time, shelve_time, delete_time }
    }

    fn beautify(&self) {
        println!("{:?}", self);
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn from_string(s: String) -> Self {
        // Here you need to implement the logic to convert a string to a Task
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(s: String) -> Self {
        serde_json::from_str(&s).unwrap()
    }
}