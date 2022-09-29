# Rest API

Our backends are usually RESTful APIs containing several functional features we would like to embed in this project.

* [x] HTTP protocol handling
* [x] Asynchronous request handling
* [ ] Implement the REST api specification (GET, POST, PUT, DELETE)
* [ ] Json serialization
* [ ] ORM tooling for connecting to the database
* [ ] Open API V3 spec / including swagger-ui.
* [ ] Containerizing our API

We already covered the first 2 requirements.

Before we go any further on the other requirements, let's create some more REST endpoints for managing our todo_items.

## Implementing all Rest methods
Typically, what I would like to do it create something like a `Controller`, which handles the request for a particilar entity.
For example: a `todo controller` would handle all incoming requests related to `todo items`.

Let's create `todo_controller.rs` and add the following functions:

#### **`todo_api/src/api/todo_controller.rs`**
```rust
#[get("/todo")]
async fn get_todos() -> impl Responder {
    format!("hello from get todos")
}

#[get("/todo/{id}")]
async fn get_todo_by_id(_id: web::Path<String>) -> impl Responder {
    format!("hello from get todos by id")
}

#[post("/todo")]
async fn create_todo() -> impl Responder {
    format!("hello from add todo")
}

#[delete("/todo/{id}")]
async fn delete_todo(_id: web::Path<String>) -> impl Responder {
    format!("hello from delete todo with id")
}

#[put("/todo/{id}")]
async fn update_todo(_id: web::Path<String>) -> impl Responder {
    format!("hello from update todos with id")
}
```

This defines handlers for the given routes and Http methods. 

## Registering the controller methods
I could go to our `main.rs` file and register each handler individually. Rather, I just register the entire controller there at once, and let the controller take care of the individual registration. This keeps the all the logic at one file, and doesn't clutter my `main.rs` file. This will help me down the line, as my project grows.

To be able to register all endpoints at once, I create a configure method, which I can call in my `main.rs` file.

#### **`todo_api/src/api/todo_controller.rs`**
```rust 
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
```
Notice that the `configure()` function, is the only publicly exposed member in this file.

#### **`todo_api/src/main.rs`**
```rust
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(api::todo_controller::configure()))
        .bind((Ipv4Addr::UNSPECIFIED, 8080))?
        .run()
        .await
}
```

Nice and clean.

## Testing our endpoints

We now have RESTful api endpoints to handle our todo management
```shell
~ curl http://localhost:8080/todo/549dd9a2-aa7a-4d29-ad4f-ded12fe02af8
hello from get todos by id

~ curl http://localhost:8080/todo
hello from get todos

~ curl -X POST http://localhost:8080/todo
hello from add todo

~ curl -X PUT http://localhost:8080/todo/549dd9a2-aa7a-4d29-ad4f-ded12fe02af8
hello from update todos with id

~ curl -X DELETE http://localhost:8080/todo/549dd9a2-aa7a-4d29-ad4f-ded12fe02af8
hello from delete todo with id
```
