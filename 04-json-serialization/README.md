# Rest API

Our backends are usually RESTful APIs containing several functional features we would like to embed in this project.

* [x] HTTP protocol handling
* [x] Asynchronous request handling
* [x] Implement the REST api specification (GET, POST, PUT, DELETE)
* [ ] Json serialization
* [ ] ORM tooling for connecting to the database
* [ ] Open API V3 spec / including swagger-ui.
* [ ] Containerizing our API

We already covered the first 3 requirements, let's see if we can get some actual todo items over the wire.
To serialize to JSON and deserialize from JSON, we will need another crate, as it is not part of the `std` library.
There are a number of Crates that can do this for us, but the most well known crate is [Serde](https://docs.rs/serde_json/latest/serde_json/), so we'll stick with that for now.

Let's add Serde to our `Cargo.Toml` file. Note that our `todo_item` stuct resides in the `todo_shared` crate. So we will need to add it there. We will use this crate for both our backend as frontend projects. Any project including the `todo_shared` crate, will directly be able to (de)serialize `todo_item` entities from and to JSON..

Additionally, we'll add the `derive` feature from Serde, to be able to make use of the device macro and keep our code nice and clean.
Important note here: in order for us to be able to (de)serialize UUID from/to Json using Serde, we'll need to the serde feature for uuid too.
#### **`todo_shared/Cargo.toml`**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = {version = "1.1.2", features = ["v4", "serde"]}
```

We can now add the Serialize and Deserialize derive macro's on top of our `todo_item` struct;

NOTE: Unfortunately, if we want to use JSON with Actixweb, we can only work with owned types, not borrowed.
(Documentation)[https://docs.rs/actix-web/3.3.2/actix_web/web/struct.Json.html#impl-FromRequest]
It indicates we can only use owned, not borrowed, data with the Json type if we want actix-web to extract types from the request for you. Thus we will have to use String for our &str members here.

#### **`todo_shared/src/models/todo_item.rs`**
```rust
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    // The unique identifier of the todo item
    pub id: Uuid,

    // The title of the todo item
    pub title: String,

    // The description of the todo item
    pub description: String,

    // Indicates whether the todo item is completed
    pub completed: bool,

    // Epoch timestamp when the todo item was completed
    pub completed_at: SystemTime,

    // Epoch timestamp when the todo item was created
    pub created_at: SystemTime,
}

// update our implementation for new accordingly
impl TodoItem {
    pub fn new(title: &str, description: &str) -> Self {
        TodoItem {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
            completed_at: SystemTime::now(),
            created_at: SystemTime::now(),
        }
    }
}
```

Let's also add to new request structs to our todo_item.rs file
#### **`todo_shared/src/models/todo_item.rs`**
```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTodoItemRequest {
    // The new title of the todo item
    pub new_title: String,

    // The new description of the todo item
    pub new_description: String,

    // Indicates whether the todo item is completed
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodoItemRequest {
    // The title of the todo item
    pub title: String,

    // The description of the todo item
    pub description: String,
}

```

We'll use these as request bodies for actually creating and updating a todo item in our controller.

Now let's add JSON input and output to our REST endpoints.


#### **`todo_api/src/api/todo_controller.rs`**
```rust
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
```

When we test our updated endpoints, we can see we can now send and receive JSON:
### Get all
```shell
~ curl http://localhost:8080/todo -s | jq
```
```json
[
  {
    "id": "549dd9a2-aa7a-4d29-ad4f-ded12fe02af8"
    "title": "Todo item 1",
    "description": "Todo item 1 body",
    "completed": false,
    "completed_at": {
      "secs_since_epoch": 1663173056,
      "nanos_since_epoch": 778128445
    },
    "created_at": {
      "secs_since_epoch": 1663173056,
      "nanos_since_epoch": 778128527
    }
  },
  {
    "id": "c2c50db4-3d8e-11ed-b878-0242ac120002",
    "title": "Todo item 2",
    "description": "Todo item 2 body",
    "completed": false,
    "completed_at": {
      "secs_since_epoch": 1663173056,
      "nanos_since_epoch": 778129744
    },
    "created_at": {
      "secs_since_epoch": 1663173056,
      "nanos_since_epoch": 778129765
    }
  }
]
```
### Get by id
```shell
~ curl http://localhost:8080/todo/c2c50db4-3d8e-11ed-b878-0242ac120002 -s | jq
```
```json
{
  "id": "c2c50db4-3d8e-11ed-b878-0242ac120002",
  "title": "going full stack on rust",
  "description": "we need more love for rust",
  "completed": false,
  "completed_at": {
    "secs_since_epoch": 1663172979,
    "nanos_since_epoch": 448129910
  },
  "created_at": {
    "secs_since_epoch": 1663172979,
    "nanos_since_epoch": 448129961
  }
}
```
### Insert a new todo item
Notice we also send JSON in our request body.
```shell
~ curl --header "Content-Type: application/json" \
       --request POST \
       --data '{"title":"xyz","description":"xyz"}' \
       http://localhost:8080/todo -s | jq
```
```json
{
  "id": "c2c50db4-3d8e-11ed-b878-0242ac120002",
  "title": "xyz",
  "description": "xyz",
  "completed": false,
  "completed_at": {
    "secs_since_epoch": 1663173189,
    "nanos_since_epoch": 965599022
  },
  "created_at": {
    "secs_since_epoch": 1663173189,
    "nanos_since_epoch": 965599069
  }
}

```
### Update an existing todo item
Notice we also send JSON in our request body.
```shell
~ curl --header "Content-Type: application/json" \
       --request PUT \
       --data '{"new_title":"xyz","new_description":"xyz", "completed": true}' \
       http://localhost:8080/todo/c2c50db4-3d8e-11ed-b878-0242ac120002 -s | jq
```
```json
{
  "id": "c2c50db4-3d8e-11ed-b878-0242ac120002",
  "title": "xyz",
  "description": "xyz",
  "completed": false,
  "completed_at": {
    "secs_since_epoch": 1663173093,
    "nanos_since_epoch": 84328132
  },
  "created_at": {
    "secs_since_epoch": 1663173093,
    "nanos_since_epoch": 84328180
  }
}
```
Nice! We can now read and write json over the wire. Up next is actually persisting our data
