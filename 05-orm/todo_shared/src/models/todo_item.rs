use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    // The unique identifier of the todo item
    pub id: i32,

    // The title of the todo item
    pub title: String,

    // The description of the todo item
    pub description: String,

    // Indicates whether the todo item is completed
    pub completed: bool,

    // Epoch timestamp when the todo item was completed
    pub completed_at: Option<SystemTime>,

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
