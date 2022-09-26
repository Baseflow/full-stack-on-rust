use crate::schema::todos;
use std::time::SystemTime;
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};

#[derive(Queryable, Insertable)]
#[diesel(table_name = todos)]
#[diesel(primary_key(id))]
pub struct TodoEntity {
    /// The unique identifier of the todo item
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,

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

impl From<TodoEntity> for TodoItem {
    fn from(entity: TodoEntity) -> Self {
        TodoItem {
            id: entity.id.unwrap_or(0),
            title: entity.title,
            description: entity.description,
            completed: entity.completed,
            completed_at: entity.completed_at,
            created_at: entity.created_at,
        }
    }
}

impl From<CreateTodoItemRequest> for TodoEntity {
    fn from(request: CreateTodoItemRequest) -> Self {
        TodoEntity {
            id: None,
            title: request.title,
            description: request.description,
            created_at: SystemTime::now(),
            completed_at: None,
            completed: false,
        }
    }
}

impl From<UpdateTodoItemRequest> for TodoEntity {
    fn from(request: UpdateTodoItemRequest) -> Self {
        TodoEntity {
            id: None,
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
