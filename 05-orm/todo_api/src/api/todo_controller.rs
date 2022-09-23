use actix_web::web::{Json, ServiceConfig};
use actix_web::HttpResponse;
use actix_web::{delete, get, post, put, web, Responder};
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};

use crate::data::repository::Repository;
use crate::data::todo_repository::TodoEntityRepository;
use crate::entities::todo_entity::TodoEntity;
use actix_web::web::Data;
use std::sync::Arc;

use log::{info, warn};

#[get("/todo")]
async fn get_todos(repository: Data<dyn Repository<TodoEntity>>) -> impl Responder {
    info!("Getting all todo items");
    // Get entities from the datastore
    let entities = repository.get_all();

    // Map our entities to our public struct TodoItem
    let response: Vec<TodoItem> = entities.into_iter().map(|entity| entity.into()).collect();

    // Send the response
    HttpResponse::Ok().json(response)
}

#[get("/todo/{id}")]
async fn get_todo_by_id(
    id: web::Path<i32>, // The identifier of the item to retrieve
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> impl Responder {
    let todo_id = id.into_inner();
    // Query our entity from the data store.
    info!("Getting todo item with id {}", todo_id);
    let entity = repository.get_by_id(todo_id);
    match entity {
        Some(item) => {
            // If we found one, use the From<T> trait to convert to a TodoItem
            let response: TodoItem = item.into();
            // Send the response
            HttpResponse::Ok().json(response)
        }
        _ => {
            warn!(
                "Todo item with id {} was not found in the data store",
                todo_id
            );
            // Let the caller know the resource was not found.
            HttpResponse::NotFound().finish()
        }
    }
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
        // Create our repository
        let repository = TodoEntityRepository::new();

        // wrap our repository in a atomic reference counter for thread safety.
        let repository_arc: Arc<dyn Repository<TodoEntity>> = Arc::new(repository);

        config
            // Register our repository for data injection;
            .app_data(Data::from(repository_arc))
            // register our endpoints
            .service(get_todos)
            .service(create_todo)
            .service(delete_todo)
            .service(get_todo_by_id)
            .service(update_todo);
    }
}
