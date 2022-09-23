use crate::data::db_context;
use crate::data::repository::Repository;
use crate::diesel::prelude::*;
use crate::entities::todo_entity::TodoEntity;
use crate::schema::todos::dsl::*;

pub struct TodoEntityRepository {
    db_context: db_context::PostgresPool,
}

impl TodoEntityRepository {
    pub fn new(db_context: db_context::PostgresPool) -> Self {
        TodoEntityRepository { db_context }
    }
}

impl Repository<TodoEntity> for TodoEntityRepository {
    fn get_all(&self) -> Vec<TodoEntity> {
        let mut connection = self.db_context.get().unwrap();
        todos
            .load::<TodoEntity>(&mut connection)
            .expect("Error loading todo items")
    }

    fn get_by_id(&self, todo_id: i32) -> Option<TodoEntity> {
        todo!();
    }

    fn insert(&self, entity: TodoEntity) -> Result<TodoEntity, String> {
        todo!();
    }

    fn update(&self, todo_id: i32, entity: TodoEntity) -> Result<TodoEntity, String> {
        todo!();
    }

    fn delete(&self, todo_id: i32) -> Result<bool, String> {
        todo!();
    }
}