use actix_web::web::{Json, ServiceConfig};
use actix_web::HttpResponse;
use actix_web::{delete, get, post, put, web, Error};
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};

use crate::data::repository::Repository;
use crate::data::todo_repository::TodoEntityRepository;
use crate::entities::todo_entity::TodoEntity;
use actix_web::web::Data;
use std::sync::Arc;
use uuid::Uuid;

use log::{error, warn};

#[get("/todo")]
async fn get_todos(repository: Data<dyn Repository<TodoEntity>>) -> Result<HttpResponse, Error> {
    // Get entities from the datastore
    let entities = web::block(move || repository.get_all())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Map our entities to our public struct TodoItem
    let response: Vec<TodoItem> = entities.into_iter().map(|entity| entity.into()).collect();

    // Send the response
    Ok(HttpResponse::Ok().json(response))
}

#[get("/todo/{id}")]
async fn get_todo_by_id(
    id: web::Path<Uuid>, // The identifier of the item to retrieve
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> Result<HttpResponse, Error> {
    let uuid = id.into_inner();

    // Query our entity from the data store.
    let entity = web::block(move || repository.get_by_id(uuid))
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match entity {
        Some(item) => {
            // If we found one, use the From<T> trait to convert to a TodoItem
            let response: TodoItem = item.into();
            // Send the response
            Ok(HttpResponse::Ok().json(response))
        }
        _ => {
            warn!("Todo item with id {} was not found in the data store", uuid);
            // Let the caller know the resource was not found.
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

#[post("/todo")]
async fn create_todo(
    todo: Json<CreateTodoItemRequest>,
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> Result<HttpResponse, Error> {
    let request_body = todo.into_inner();
    let result = web::block(move || repository.insert(request_body.into()))
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(entity) => {
            let result: TodoItem = entity.into();
            Ok(HttpResponse::Ok().json(result))
        }
        _ => {
            error!("Unable to insert new todo item");
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[delete("/todo/{id}")]
async fn delete_todo(
    id: web::Path<Uuid>,
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> Result<HttpResponse, Error> {
    let result = web::block(move || repository.delete(id.into_inner()))
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    match result {
        Ok(true) => Ok(HttpResponse::Ok().finish()),
        _ => Ok(HttpResponse::NotFound().finish()),
    }
}

#[put("/todo/{id}")]
async fn update_todo(
    id: web::Path<Uuid>,
    todo: Json<UpdateTodoItemRequest>,
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> Result<HttpResponse, Error> {
    let request_body = todo.into_inner();
    let uuid = id.into_inner();
    let entity = web::block(move || repository.update(uuid, request_body.into()))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let result: TodoItem = entity.into();
    Ok(HttpResponse::Ok().json(result))
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

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use actix_web::{test, App};
    use uuid::Uuid;

    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::Mutex;

    use crate::data::repository::Repository;
    use crate::entities::todo_entity::TodoEntity;

    use super::*;

    // Define a mock for Repository<TodoEntity>
    pub struct TodoEntityRepositoryMock {
        db: Arc<Mutex<HashMap<Uuid, TodoEntity>>>,
    }

    // Implement our repository pattern for the mock.
    impl Repository<TodoEntity> for TodoEntityRepositoryMock {
        fn get_all(&self) -> Vec<TodoEntity> {
            self.db
                .lock()
                .unwrap()
                .values()
                .map(|v| v.clone())
                .collect()
        }

        fn get_by_id(&self, todo_id: Uuid) -> Option<TodoEntity> {
            self.db.lock().unwrap().get(&todo_id).map(|f| f.clone())
        }

        fn insert<'a>(&self, entity: TodoEntity) -> Result<TodoEntity, String> {
            self.db.lock().unwrap().insert(entity.id, entity.clone());
            Ok(entity)
        }

        fn update(&self, todo_id: Uuid, entity: TodoEntity) -> Result<TodoEntity, String> {
            *self.db.lock().unwrap().get_mut(&todo_id).unwrap() = entity.clone();
            Ok(entity)
        }

        fn delete(&self, todo_id: Uuid) -> Result<bool, String> {
            self.db.lock().unwrap().remove(&todo_id);
            Ok(true)
        }
    }

    fn get_repository_mock_with_data() -> Arc<dyn Repository<TodoEntity>> {
        // Create our repository
        let repository = TodoEntityRepositoryMock {
            db: Arc::new(Mutex::new(HashMap::new())),
        };

        // insert some mock data
        let _ = repository.insert(TodoEntity {
            id: Uuid::parse_str("cdce7fda-909e-41cb-8507-abceb316a5b4").unwrap(),
            title: "Test the microservice".to_string(),
            description: "We should test the get all method".to_string(),
            completed: true,
            completed_at: Some(SystemTime::now()),
            created_at: SystemTime::now(),
        });
        let _ = repository
            .insert(TodoEntity {
                id: Uuid::parse_str("120400b8-eee8-47cc-9e96-5bc0a3e2e874").unwrap(),
                title: "Use a mock repository".to_string(),
                description: "We should test that we can also use a mock for the same handler"
                    .to_string(),
                completed: true,
                completed_at: Some(SystemTime::now()),
                created_at: SystemTime::now(),
            })
            .unwrap();

        let repository_arc: Arc<dyn Repository<TodoEntity>> = Arc::new(repository);
        repository_arc
    }

    #[actix_web::test]
    async fn test_get_all() {
        let app = test::init_service(
            App::new()
                .app_data(Data::from(get_repository_mock_with_data()))
                .service(get_todos),
        )
        .await;
        let req = test::TestRequest::default().uri("/todo").to_request();

        let resp: Vec<TodoItem> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.len(), 2);
    }

    #[actix_web::test]
    async fn test_get_by_id() {
        let repository = get_repository_mock_with_data();
        let app = test::init_service(
            App::new()
                .app_data(Data::from(repository))
                .service(get_todo_by_id),
        )
        .await;

        let req = test::TestRequest::default()
            .uri("/todo/120400b8-eee8-47cc-9e96-5bc0a3e2e874")
            .to_request();

        let resp: TodoItem = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.title, "Use a mock repository");
        assert_eq!(
            resp.description,
            "We should test that we can also use a mock for the same handler"
        );
        assert_eq!(resp.completed, true);
    }

    #[actix_web::test]
    async fn test_create_todo() {
        let repository = get_repository_mock_with_data();
        let app = test::init_service(
            App::new()
                .app_data(Data::from(repository))
                .service(create_todo),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/todo")
            .set_json(&CreateTodoItemRequest {
                title: "Test create".to_string(),
                description: "We should test the create method".to_string(),
            })
            .to_request();

        let resp: TodoItem = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.title, "Test create");
        assert_eq!(resp.description, "We should test the create method");
        assert_eq!(resp.completed, false);
    }

    #[actix_web::test]
    async fn test_update_todo() {
        let repository = get_repository_mock_with_data();
        let app = test::init_service(
            App::new()
                .app_data(Data::from(repository))
                .service(update_todo),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/todo/120400b8-eee8-47cc-9e96-5bc0a3e2e874")
            .set_json(&UpdateTodoItemRequest {
                new_title: "Test update".to_string(),
                new_description: "We should test the update method".to_string(),
                completed: true,
            })
            .to_request();

        let resp: TodoItem = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.title, "Test update");
        assert_eq!(resp.description, "We should test the update method");
        assert_eq!(resp.completed, true);
    }

    #[actix_web::test]
    async fn test_delete_todo() {
        let repository = get_repository_mock_with_data();
        let app = test::init_service(
            App::new()
                .app_data(Data::from(repository))
                .service(delete_todo)
                .service(get_todos),
        )
        .await;

        let initial_req = test::TestRequest::default().uri("/todo").to_request();
        let resp: Vec<TodoItem> = test::call_and_read_body_json(&app, initial_req).await;

        assert_eq!(resp.len(), 2);

        let delete_req = test::TestRequest::delete()
            .uri("/todo/120400b8-eee8-47cc-9e96-5bc0a3e2e874")
            .to_request();

        let _ = test::call_and_read_body(&app, delete_req).await;

        let validation_req = test::TestRequest::default().uri("/todo").to_request();
        let resp: Vec<TodoItem> = test::call_and_read_body_json(&app, validation_req).await;
        assert_eq!(resp.len(), 1);
    }
}
