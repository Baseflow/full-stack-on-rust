use actix_web::web::ServiceConfig;
use actix_web::{delete, get, post, put, web, Responder};

#[get("/todo")]
async fn get_todos() -> impl Responder {
    format!("hello from get todos")
}

#[get("/todo/{id}")]
async fn get_todo_by_id(_id: web::Path<u32>) -> impl Responder {
    format!("hello from get todos by id")
}

#[post("/todo")]
async fn create_todo() -> impl Responder {
    format!("hello from add todo")
}

#[delete("/todo/{id}")]
async fn delete_todo(_id: web::Path<u32>) -> impl Responder {
    format!("hello from delete todo")
}

#[put("/todo/{id}")]
async fn update_todo(_id: web::Path<u32>) -> impl Responder {
    format!("hello from update todos with id")
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
