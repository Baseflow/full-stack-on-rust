use actix_web::web::{Json, ServiceConfig};
use actix_web::HttpResponse;
use actix_web::{delete, get, post, put, web, Responder};
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};

use crate::data::repository::Repository;
use crate::data::todo_repository::TodoEntityRepository;
use crate::entities::todo_entity::TodoEntity;
use actix_web::web::Data;
use std::sync::Arc;

use log::{error, info, warn};

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
async fn create_todo(
    todo: Json<CreateTodoItemRequest>,
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> impl Responder {
    let request_body = todo.into_inner();
    let result = repository.insert(request_body.into());
    match result {
        Ok(entity) => {
            let result: TodoItem = entity.into();
            HttpResponse::Ok().json(result)
        }
        _ => {
            error!("Unable to insert new todo item");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/todo/{id}")]
async fn delete_todo(
    id: web::Path<i32>,
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> impl Responder {
    let todo_id = id.into_inner();
    let result = repository.delete(todo_id);
    match result {
        Ok(success) => match success {
            true => HttpResponse::Ok().finish(),
            _ => HttpResponse::NotFound().finish(),
        },
        _ => {
            error!("Unable to delete item");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/todo/{id}")]
async fn update_todo(
    id: web::Path<i32>,
    todo: Json<UpdateTodoItemRequest>,
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> impl Responder {
    let request_body = todo.into_inner();
    let todo_id = id.into_inner();
    let result = repository.update(todo_id, request_body.into());
    match result {
        Ok(entity) => {
            let result: TodoItem = entity.into();
            HttpResponse::Ok().json(result)
        }
        _ => {
            error!("Unable to update existing todo item");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        // Create our repository
        let repository = TodoEntityRepository::new();

        // Todo entity repository is unsized, so we need to wrap this in a Atomic Reference Counter
        // "For types that are unsized, most commonly dyn T, Data can wrap these types by first constructing an Arc<dyn T> and using the From implementation to convert it."
        // https://docs.rs/actix-web/latest/actix_web/web/struct.Data.html
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
