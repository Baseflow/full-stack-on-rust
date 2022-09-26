use crate::schema::todos;
use std::time::SystemTime;
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = todos)]
#[diesel(primary_key(id))]
pub struct TodoEntity {
    /// The unique identifier of the todo item
    pub id: Uuid,

    /// The title of the todo item
    pub title: String,

    /// The description of the todo item
    pub description: String,

    /// Indicates whether the todo item is completed
    pub completed: bool,

    /// Timestamp when the todo item was completed
    pub completed_at: Option<SystemTime>,

    /// Timestamp when the todo item was created
    pub created_at: SystemTime,
}

// Convert from TodoEntity to TodoItem
impl From<TodoEntity> for TodoItem {
    fn from(entity: TodoEntity) -> Self {
        TodoItem {
            id: entity.id,
            title: entity.title,
            description: entity.description,
            completed: entity.completed,
            completed_at: entity.completed_at,
            created_at: entity.created_at,
        }
    }
}

// Convert from CreateTodoItemRequest to TodoEntity
impl From<CreateTodoItemRequest> for TodoEntity {
    fn from(request: CreateTodoItemRequest) -> Self {
        TodoEntity {
            id: Uuid::new_v4(),
            title: request.title,
            description: request.description,
            created_at: SystemTime::now(),
            completed_at: None,
            completed: false,
        }
    }
}

// Convert from UpdateTodoItemRequest to TodoEntity
impl From<UpdateTodoItemRequest> for TodoEntity {
    fn from(request: UpdateTodoItemRequest) -> Self {
        TodoEntity {
            id: Uuid::new_v4(),
            title: request.new_title,
            description: request.new_description,
            created_at: SystemTime::now(),
            completed_at: match request.completed {
                true => Some(SystemTime::now()),
                _ => None,
            },
            completed: request.completed,
        }
    }
}
