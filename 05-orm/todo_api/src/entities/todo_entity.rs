use std::time::SystemTime;
use todo_shared::TodoItem;

#[derive(Queryable)]
pub struct TodoEntity {
    /// The unique identifier of the todo item
    pub id: i32,

    /// The title of the todo item
    pub title: String,

    /// The description of the todo item
    pub description: String,

    /// Indicates whether the todo item is completed
    pub completed: bool,

    /// Timestamp when the todo item was completed
    pub completed_at: SystemTime,

    /// Timestamp when the todo item was created
    pub created_at: SystemTime,
}

impl From<TodoEntity> for TodoItem {
    fn from(entity: TodoEntity) -> Self {
        TodoItem {
            id: entity.id.into(),
            title: entity.title,
            description: entity.description,
            completed: entity.completed,
            completed_at: entity.completed_at,
            created_at: entity.created_at,
        }
    }
}
