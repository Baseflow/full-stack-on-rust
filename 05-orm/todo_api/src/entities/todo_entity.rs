use chrono::NaiveDateTime;
#[derive(Queryable)]
pub struct TodoEntity {
    // The unique identifier of the todo item
    pub id: i32,

    // The title of the todo item
    pub title: String,

    // The description of the todo item
    pub description: String,

    // Indicates whether the todo item is completed
    pub completed: bool,

    // Epoch timestamp when the todo item was completed
    pub completed_at: NaiveDateTime,

    // Epoch timestamp when the todo item was created
    pub created_at: NaiveDateTime,
}
