use actix_web::web::{Json, ServiceConfig};
use actix_web::HttpResponse;
use actix_web::{delete, get, post, put, web, Responder};
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};

#[get("/todo")]
async fn get_todos() -> impl Responder {
    let response = vec![
        TodoItem::new("Todo item 1", "Todo item 1 body"),
        TodoItem::new("Todo item 2", "Todo item 2 body"),
    ];
    HttpResponse::Ok().json(response)
}

#[get("/todo/{id}")]
async fn get_todo_by_id(_id: web::Path<String>) -> impl Responder {
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
async fn delete_todo(_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[put("/todo/{id}")]
async fn update_todo(_id: web::Path<String>, todo: Json<UpdateTodoItemRequest>) -> impl Responder {
    HttpResponse::Ok().json(TodoItem::new(&todo.new_title, &todo.new_description))
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(get_todos)
            .service(create_todo)
            .service(delete_todo)
            .service(get_todo_by_id)
            .service(update_todo);
    }
}
