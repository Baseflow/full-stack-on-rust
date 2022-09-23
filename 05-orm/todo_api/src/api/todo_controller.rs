use actix_web::web::{Json, ServiceConfig};
use actix_web::HttpResponse;
use actix_web::{delete, get, post, put, web, Responder};
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};

use crate::data::{db_context, repository::Repository, todo_repository};
use crate::entities::todo_entity::TodoEntity;
use actix_web::web::Data;

#[get("/todo")]
async fn get_todos(repository: Data<dyn Repository<TodoEntity>>) -> impl Responder {
    let response = vec![
        TodoItem::new("Todo item 1", "Todo item 1 body"),
        TodoItem::new("Todo item 2", "Todo item 2 body"),
    ];
    let result = repository.get_all();
    HttpResponse::Ok().json(response)
}

#[get("/todo/{id}")]
async fn get_todo_by_id(_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().json(TodoItem::new(
        "going full stack on rust",
        "we need more love for rust",
    ))
}

#[post("/todo")]
async fn create_todo(todo: Json<CreateTodoItemRequest>) -> impl Responder {
    HttpResponse::Ok().json(TodoItem::new(&todo.title, &todo.description))
}

#[delete("/todo/{id}")]
async fn delete_todo(_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[put("/todo/{id}")]
async fn update_todo(_id: web::Path<u32>, todo: Json<UpdateTodoItemRequest>) -> impl Responder {
    HttpResponse::Ok().json(TodoItem::new(&todo.new_title, &todo.new_description))
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            // Register our repository;
            .app_data(todo_repository::TodoEntityRepository::new(
                db_context::get_pool(),
            ))
            // register our endpoints
            .service(get_todos)
            .service(create_todo)
            .service(delete_todo)
            .service(get_todo_by_id)
            .service(update_todo);
    }
}
