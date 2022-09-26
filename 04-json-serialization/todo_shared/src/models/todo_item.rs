use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    // The unique identifier of the todo item
    pub id: Uuid,

    // The title of the todo item
    pub title: String,

    // The description of the todo item
    pub description: String,

    // Indicates whether the todo item is completed
    pub completed: bool,

    // Epoch timestamp when the todo item was completed
    pub completed_at: SystemTime,

    // Epoch timestamp when the todo item was created
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTodoItemRequest {
    // The new title of the todo item
    pub new_title: String,

    // The new description of the todo item
    pub new_description: String,

    // Indicates whether the todo item is completed
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodoItemRequest {
    // The title of the todo item
    pub title: String,

    // The description of the todo item
    pub description: String,
}

impl TodoItem {
    pub fn new(title: &str, description: &str) -> Self {
        TodoItem {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
            completed_at: SystemTime::now(),
            created_at: SystemTime::now(),
        }
    }
}
