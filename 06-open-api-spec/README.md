# Open API V3 Spec

Our backends are usually RESTful APIs containing several functional features we would like to embed in this project.

* [x] HTTP protocol handling
* [x] Asynchronous request handling
* [x] Implement the REST API specification (GET, POST, PUT, DELETE)
* [x] JSON serialization
* [x] ORM tooling for connecting to the database
* [ ] Open API V3 spec / including swagger-ui.
* [ ] Containerizing our API

We already covered the first 5 requirements, let's see if add generate and serve an automatically generated Open API V3 spec from our source code.
There aren't a lot of crates out there that support automatic Open API V3 generation, especially compatible with Actix-web. [Utoipa](https://crates.io/crates/utoipa/2.2.0) seems to be the most complete crate available, which also supports various web frameworks. Let's use this for now.

Add the following dependencies to the Cargo.toml file in our API project:

#### **`todo_api/Cargo.toml`**
```toml
utoipa = { version = "^2.2.0", features = ["actix_extras"] }
utoipa-swagger-ui = {version = "^2.2.0", features = ["actix-web"]}
```

We'll also need to add some Utoipa to our shared project, where our interface models reside.
#### **`todo_api/Cargo.toml`**
```toml
utoipa = "^2.2.0"
```

## Including our public models to the Open API V3 spec
Now that we've loaded `Utoipa` to our shared project, we can make the models a part of the open API spec. It's pretty straightforward. Add `use utoipa::ToSchema;` to the top of the `todo_item.rs` file and add the derive macro for 'ToSchema' to our structs:

```rust
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct TodoItem {
    ...
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UpdateTodoItemRequest {
    ...
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateTodoItemRequest {
    ...
}
```

## Including information about our endpoints
We should also add information regarding our endpoints to the Open API spec. Utoipa provides us the macro's to do so. We should also add nice documentation to our controllers, which 

#### **`todo_api/src/api/todo_controller.rs`**
```rust
/// Get list of todos.
///
/// List todos from the todo store.
#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items", body = [TodoItem])
    )
)]
#[get("/todo")]
async fn get_todos(repository: Data<dyn Repository<TodoEntity>>) -> impl Responder {
    ...
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
    ...
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
    ...
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
    ...
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
    ...
}
```

## Configure Open API

Now that we have defined our endpoints, we can start wiring up Utoipa. 
First, let's configure Utoipa in our API module.

#### **`todo_api/src/api/mod.rs`**
```rust
use utoipa::OpenApi;
use todo_shared::{CreateTodoItemRequest, TodoItem, UpdateTodoItemRequest};

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
```

## Setup swagger-ui
And finally, we'll add an endpoint for out swagger-ui in our main.rs file.

#### **`todo_api/src/main.rs`**
```rust
use utoipa_swagger_ui::SwaggerUi;

async fn main() -> std::io::Result<()> {
    ... other startup logic ...

    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = api::register_open_api_spec();

    HttpServer::new(move || {
        App::new()
            .configure(api::todo_controller::configure())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
```
> Note that to force the closure to take ownership of `openapi` (and any other referenced variables), we have to use the `move` keyword

And when we navigate to [localhost](http://localhost:8080/swagger-ui/) we'll be presented with a very nice swagger-ui page.
![image](https://user-images.githubusercontent.com/35781348/192316201-065370b4-56f7-434e-b99a-490ac82ad7fa.png)

## BONUS
Do you have third parties integrating with your backend. Here is why the open-api spec is so powerful:
Navigate to [Swagger editor](https://editor-next.swagger.io/) and paste in the yaml definition from the Open API spec. 
From here you can just generate a client for all sorts of programming languages. This will make integration for your 3rd parties much easier.
> Note, I'm currently unable to output the yaml definition yet, but converting online van [JSON > YAML](https://www.json2yaml.com/) works pretty well.
> Will update this repository later with a YAML example.
