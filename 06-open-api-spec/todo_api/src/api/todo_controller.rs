use actix_web::web::{Json, ServiceConfig};
use actix_web::HttpResponse;
use actix_web::{delete, get, post, put, web, Responder};
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};

use crate::data::repository::Repository;
use crate::data::todo_repository::TodoEntityRepository;
use crate::entities::todo_entity::TodoEntity;
use actix_web::web::Data;
use std::sync::Arc;
use uuid::Uuid;

use log::{error, warn};

/// Get list of todos.
///
/// List todos from in-memory todo store.
/// One could call the api endpoit with following curl.
#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items", body = [TodoItem])
    )
)]
#[get("/todo")]
async fn get_todos(repository: Data<dyn Repository<TodoEntity>>) -> impl Responder {
    // Get entities from the datastore
    let entities = repository.get_all();

    // Map our entities to our public struct TodoItem
    let response: Vec<TodoItem> = entities.into_iter().map(|entity| entity.into()).collect();

    // Send the response
    HttpResponse::Ok().json(response)
}

/// Get Todo by given todo id.
///
/// Return found `Todo` with status 200 or 404 not found if `Todo` is not found in the data store.
#[utoipa::path(
    responses(
        (status = 200, description = "Todo found from storage", body = TodoItem),
        (status = 400, description = "The given identifier was not a correct uuid"),
        (status = 404, description = "Todo item was not found with the given identifier"),
    ),
    params(
        ("id", description = "Unique storage id of Todo")
    )
)]
#[get("/todo/{id}")]
async fn get_todo_by_id(
    id: web::Path<String>, // The identifier of the item to retrieve
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> impl Responder {
    let todo_id = Uuid::parse_str(&id.into_inner());

    match todo_id {
        Ok(uuid) => {
            // Query our entity from the data store.
            let entity = repository.get_by_id(uuid);
            match entity {
                Some(item) => {
                    // If we found one, use the From<T> trait to convert to a TodoItem
                    let response: TodoItem = item.into();
                    // Send the response
                    HttpResponse::Ok().json(response)
                }
                _ => {
                    warn!("Todo item with id {} was not found in the data store", uuid);
                    // Let the caller know the resource was not found.
                    HttpResponse::NotFound().finish()
                }
            }
        }
        _ => {
            warn!("User supplied an unvalid uuid");
            HttpResponse::BadRequest().finish()
        }
    }
}

/// Create new Todo to the data source.
///
/// Post a new `Todo` in request body as json to store it. Api will return
/// created `Todo` on success or `ErrorResponse::InternalServerError` if a problem occured whilst creating the todo item.
#[utoipa::path(
    request_body = CreateTodoItemRequest,
    responses(
        (status = 201, description = "Todo created successfully", body = Todo),
        (status = 500, description = "Unable to insert new todo item", body = ErrorResponse)
    )
)]
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

/// Delete Todo by given path variable id.
///
/// Api will delete todo from datasource by the provided id and return success 200.
/// If storage does not contain `Todo` with given id 404 not found will be returned.
#[utoipa::path(
    responses(
        (status = 200, description = "Todo deleted successfully"),
        (status = 400, description = "The given identifier was not a correct uuid"),
        (status = 404, description = "Todo item was not found with the given identifier"),
        (status = 500, description = "Unable to delete todo item", body = ErrorResponse)
    ),
    params(
        ("id", description = "Unique storage id of Todo")
    ),
)]
#[delete("/todo/{id}")]
async fn delete_todo(
    id: web::Path<String>,
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> impl Responder {
    let todo_id = Uuid::parse_str(&id.into_inner());
    match todo_id {
        Ok(uuid) => {
            let result = repository.delete(uuid);
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
        _ => {
            warn!("User supplied an unvalid uuid");
            HttpResponse::BadRequest().finish()
        }
    }
}

/// Update Todo with given id.
///
/// Tries to update `Todo` by given id as path variable. If todo is found by id values are
/// updated according `TodoUpdateRequest` and updated `Todo` is returned with status 200.
/// If todo is not found then 404 not found is returned.
#[utoipa::path(
    request_body = TodoUpdateRequest,
    responses(
        (status = 200, description = "Todo updated successfully", body = TodoItem),
        (status = 400, description = "The given identifier was not a correct uuid"),
        (status = 404, description = "Todo item was not found with the given identifier"),
        (status = 500, description = "Unable to delete todo item", body = ErrorResponse)
    ),
    params(
        ("id", description = "Unique storage id of Todo")
    ),
)]
#[put("/todo/{id}")]
async fn update_todo(
    id: web::Path<String>,
    todo: Json<UpdateTodoItemRequest>,
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> impl Responder {
    let request_body = todo.into_inner();

    let todo_id = Uuid::parse_str(&id.into_inner());
    match todo_id {
        Ok(uuid) => {
            let result = repository.update(uuid, request_body.into());
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
        _ => {
            warn!("User supplied an unvalid uuid");
            HttpResponse::BadRequest().finish()
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
