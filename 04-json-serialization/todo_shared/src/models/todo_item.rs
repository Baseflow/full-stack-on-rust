use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoItem<'a> {
    // The unique identifier of the todo item
    pub id: u32,

    // The title of the todo item
    pub title: &'a str,

    // The description of the todo item
    pub description: &'a str,

    // Indicates whether the todo item is completed
    pub completed: bool,

    // Epoch timestamp when the todo item was completed
    pub completed_at: SystemTime,

    // Epoch timestamp when the todo item was created
    pub created_at: SystemTime,
}

impl<'a> TodoItem<'a> {
    pub fn new(title: &'a str, description: &'a str) -> Self {
        TodoItem {
            id: 0,
            title,
            description,
            completed: false,
            completed_at: SystemTime::now(),
            created_at: SystemTime::now(),
        }
    }
}
