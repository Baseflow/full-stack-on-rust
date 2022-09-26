pub mod todo_controller;
pub use todo_controller::configure;
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};
use utoipa::OpenApi;

pub fn register_open_api_spec() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            todo_controller::get_todos,
            todo_controller::get_todo_by_id,
            todo_controller::create_todo,
            todo_controller::update_todo,
            todo_controller::delete_todo,
        ),
        components(
            schemas(TodoItem, UpdateTodoItemRequest, CreateTodoItemRequest)
        ),
        tags(
            (name = "todo", description = "Todo management endpoints.")
        )
    )]

    struct ApiDoc;
    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();
    openapi
}
