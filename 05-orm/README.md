# ORM

Our backends are usually RESTful APIs containing several functional features we would like to embed in this project.

* [x] HTTP protocol handling
* [x] Asynchronous request handling
* [x] Implement the REST API specification (GET, POST, PUT, DELETE)
* [x] JSON serialization
* [ ] ORM tooling for connecting to the database
* [ ] Open API V3 spec / including swagger-ui.
* [ ] Containerizing our API

We already covered the first 4 requirements, let's see if we can persist our data in a database.
For communication with the database, ORM (Object Relational Mapper) tooling is usually the way we interact with our database.
ORM has a couple of advantages: it abstracts actual querying to the database for us. Whether we will be using Postsqres or SqLite, the specifics of that interfacing with a particular type of database will be handled by the ORM tooling. We are not bothered by it in our codebase. 
Additionally, ORM tooling will take care of serializing/deserializing objects from and to the database. ORM tooling can also be used to keep our database up to date with migrations, which simplifies DevOps. 

There are a couple of ORM crates available for Rust.
* Diesel
* Sea-orm
* Rustorm

Diesel is by far the most well-known ORM crate with over 4.5 million downloads. It supports SqLite, Postsqres, and MySql out of the box, but can be extended with other database engines. We'll stick with that for our full-stack adventure.

## Diesel
Let's add diesel to our API project. We'll also add `dotenvy` to make use of environment variables. 
For connection pooling, we'll use `r2d2`. Additionally, we'll also add `diesel_migrations` for automatic migration deployment upon application startup.
Out of the box, uuid's are not serialized by diesel, but adding the 'uuid' feature to diesel will supply us with the logic to do so.
Finally, we'll add `env_logger` and `log` to add a logging framework.

#### **`todo_api/Cargo.toml`**
```toml
[dependencies]
diesel = { version = "2.0.0", features = ["postgres", "r2d2", "uuid"] }
dotenv = "0.15.0"
diesel_migrations = "1.4.0"
r2d2 = "0.8.9"
env_logger = "0.9.0"
log = "0.4.17"
uuid = {version = "1.1.2", features = ["v4"]}
```

You should also install the `diesel_cli` binary to make use of the CLI tooling diesel offers, to create migrations and schemas.
In this case, we are only interested in interfacing with Postgres, so don't compile tooling for other database engines just yet.


```shell
cargo install diesel_cli --no-default-features --features postgres
```

I've added a docker-compose file that hosts postgres on a non-default port. You can just run this to run Postgres, and it will most likely not conflict with other postgres instances you might have running on your machine.

Navigate to the **todo_api/docker/postgres** folder and run 

```shell
docker-compose up -d && docker-compose logs -f
```
This will spin up a docker image with Postgres on port 5555. It runs in a detached mode, to quitting the command using `ctrl + c` will not kill the container.

Now that our database is running, we can hook up diesel to it.
Let's start by setting some environment variables for Diesel to tell which endpoint to connect with diesel, and what credentials to use.
Navigate to the **todo_api** directory and run the following command:

```shell
echo DATABASE_URL=postgres://full-stack:on-rust@localhost:5555/todo_api > .env
```

We can let diesel_cli create the database for us using `diesel setup`. This will also validate if Diesel can connect to the database using the DATABASE_URL from the environment variables.
Additionally, a migrations folder is already created with an 'Initial setup' migration.

Migrations always contain up and down scripts, used for applying and un-applying migrations as you want.

## Adding tables to the database
As I mentioned before, ORM tooling can be useful to create/manage migrations, and have them applied to the database for us.
Diesel_cli can be used to create migrations.
```shell
diesel migration generate create_todo_table
```
This will create a new migration in the migrations folder.
Here we can specify what the required actions are for applying the migration, and undoing the migration.

For example: if we want to apply the migration, we'll need to create a todo table to the database.
If we want to roll back this migration, we want to delete the todo table.

Navigate to the newly created `create_todo_table` folder and add the following statement to the `up.sql` and `down.sql` file:

#### **`todo_api/migrations/2022-09-23-122632_create_todo_table/up.sql`**
```sql
CREATE TABLE todos (
  id UUID PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT 'f',
  completed_at TIMESTAMP,
  created_at TIMESTAMP
)
```

#### **`todo_api/migrations/2022-09-23-122632_create_todo_table/down.sql`**
```sql
DROP TABLE todos
```

We can apply our new migration using `diesel migration run`.

## Query todo items
To make sure our item can be queried, serialized, and deserialized from/to the database, we'll need to add the `Queryable` derive macro to our todo_item.
However, the current todo_item model we have represent something that defines our public interface from/to the API and the clients. No client should have to depend on diesel to function.
Therefore, we'll create a new todo_item entity, which represents the todo_api as returned/send to the database. Only our API project knows about the underlying data source, and how to interact with that. We can use mapping later to map our entity to the public interface model, returned by our API to the clients.
You'll notice that it looks very similar, but it has different functions. Serde is not used here for JSON serialization as we don't require that here. However, we included the `Queryable` and `Insertable` macro to interact with the database.

#### **`todo_api/src/entities/todo_item.rs`**
```rust
#[derive(Queryable)]
use crate::schema::todos;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, Insertable, Clone)]
#[diesel(table_name = todos)]
#[diesel(primary_key(id))]
pub struct TodoEntity {
    /// The unique identifier of the todo item
    pub id: Uuid,

    /// The title of the todo item
    pub title: String,

    /// The description of the todo item
    pub description: String,

    /// Indicates whether the todo item is completed
    pub completed: bool,

    /// Timestamp when the todo item was completed
    pub completed_at: Option<SystemTime>,

    /// Timestamp when the todo item was created
    pub created_at: SystemTime,
}
```

## Connection pooling
Creating a database connection can easily be done with Diesel. However, creating a new connection for every incoming request doesn't scale all that well. Additionally, Postgresql only allows 100 connections to be opened by default to the server, and will, with high loads, eventually block new connections from being created.

To solve this, we'll be using connection pooling. We'll keep a maximum of let's say, 10 connections, alive and re-use them. This scales a lot better as the load on our server increases over time.

Let's create a **db_context.rs** file that handles all the connection pooling for us:
```rust
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

// The Postgres-specific connection pool managing all database connections.
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PostgresPool {
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let migr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}
```
## Repository pattern
This part is more of a personal preference, but I usually use a repository pattern for my data layer interfacing. My controllers and business logic should not be aware of where the data is stored, but rather just know that there is 'a' place (repository) where todo items are stored. I should be able to interchange repositories implementations by changing registrations, rather than changing business logic all over the place. Again, this is personal, as an ORM already abstracts a lot of stuff for us. Don't follow up on my advice if you don't want to.

Let's create a repository pattern by defining the `Repository<T>` trait. It defines a generic abstraction for basic CRUD actions on a data source.

#### **`todo_api/src/data/repository.rs`**
```rust
pub trait Repository<T> {
    /// Returns all availble instances of `<T>`
    fn get_all(&self) -> Vec<T>;

    /// Returns a single instance of `<T>` based on the given id
    ///
    ///  # Arguments
    ///  
    ///  * `id` - The identifier of the item to find in the data store.
    fn get_by_id(&self, id: uuid::Uuid) -> Option<T>;

    /// Inserts a single instance of `<T>` in the data store
    ///
    ///  # Arguments
    ///  
    ///  * `entity` - The entity to insert.
    fn insert(&self, entity: T) -> Result<T, String>;

    /// Updates a single instance of `<T>` in the data store with the given `id`
    ///
    ///  # Arguments
    ///  
    ///  * `id` - The unique identifier of the entity to update
    ///  * `entity` - An updated version of the entity with the latest values.
    fn update(&self, id: uuid::Uuid, entity: T) -> Result<T, String>;

    /// Deletes a single instance of `<T>` from the data store with the given `id`
    ///
    ///  # Arguments
    ///  
    ///  * `id` - The identifier of the item to delete from the data store.
    fn delete(&self, id: uuid::Uuid) -> Result<bool, String>;
}
```

We can now add a database-specific implementation for Repository<TodoItem> which acts as a Repository for TodoItems that communicates with a database. We could interchange this with an InMemory Repository in case we want to perform unit tests, for example.

#### **`todo_api/src/data/todo_repository.rs`**
```rust
use uuid::Uuid;

use crate::data::db_context;
use crate::data::repository::Repository;
use crate::diesel::prelude::*;
use crate::entities::todo_entity::TodoEntity;
use crate::schema::todos;
use crate::schema::todos::dsl::*;

pub struct TodoEntityRepository {
    db_context: db_context::PostgresPool,
}

impl TodoEntityRepository {
    pub fn new() -> Self {
        TodoEntityRepository {
            db_context: db_context::get_pool(),
        }
    }
}

impl Repository<TodoEntity> for TodoEntityRepository {
    fn get_all(&self) -> Vec<TodoEntity> {
        let mut connection = self.db_context.get().unwrap();
        todos
            .load::<TodoEntity>(&mut connection)
            .expect("Error loading todo items")
    }

    fn get_by_id(&self, todo_id: Uuid) -> Option<TodoEntity> {
        let mut connection = self.db_context.get().unwrap();
        let item = todos.find(todo_id).first(&mut connection);
        if item.is_ok() {
            Some(item.unwrap())
        } else {
            None
        }
    }

    fn insert<'a>(&self, entity: TodoEntity) -> Result<TodoEntity, String> {
        let mut connection = self.db_context.get().unwrap();
        let result = diesel::insert_into(todos::table)
            .values(entity)
            .get_result::<TodoEntity>(&mut connection)
            .expect("Unable to insert todo item into database");
        Ok(result)
    }

    fn update(&self, todo_id: Uuid, entity: TodoEntity) -> Result<TodoEntity, String> {
        let mut connection = self.db_context.get().unwrap();
        let todo_item = diesel::update(todos.find(todo_id))
            .set((
                completed_at.eq(entity.completed_at),
                completed.eq(entity.completed),
                title.eq(entity.title),
                description.eq(entity.description),
            ))
            .get_result::<TodoEntity>(&mut connection)
            .expect("Unable to update todo entity");

        Ok(todo_item)
    }

    fn delete(&self, todo_id: Uuid) -> Result<bool, String> {
        let mut connection = self.db_context.get().unwrap();
        let num_deleted = diesel::delete(todos.find(todo_id))
            .execute(&mut connection)
            .expect("Error deleting todo item with id {}");
        Ok(num_deleted > 0)
    }
}
```
As you can see, this specific implementation of `Repository<TodoEntity>` uses the connection pool from Diesel, and performs diesel-specific operations for interfacing with the database.

## Using the repository

Actix-web can perform some degree of dependency injection, which is terrific. Whenever our endpoints get called, We don't want to be bothered with setting up the repository and its database connections. Additionally, we want to work with abstractions, not specific implementations.

To have our Repository<TodoEntity> injected into our handlers, we need to register is within the App_Data:
Let's alter the todo_controller `configure` method:

#### **`todo_api/src/api/todo_controller.rs`**
```rust

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
```

We can now use our registered repository in our controllers like this:

#### **`todo_api/src/api/todo_controller.rs`**
```rust
#[get("/todo/{id}")]
async fn get_todo_by_id(
    id: web::Path<String>, // The identifier of the item to retrieve
    repository: Data<dyn Repository<TodoEntity>>, // The todo item repository, injected from app_data
) -> impl Responder {
    ...
}
```
Now that we have our endpoint controllers, and our repository ready, let's implement our controller logic:

#### **`todo_api/src/api/todo_controller.rs`**
```rust
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

#[get("/todo")]
async fn get_todos(repository: Data<dyn Repository<TodoEntity>>) -> impl Responder {
    // Get entities from the datastore
    let entities = repository.get_all();

    // Map our entities to our public struct TodoItem
    let response: Vec<TodoItem> = entities.into_iter().map(|entity| entity.into()).collect();

    // Send the response
    HttpResponse::Ok().json(response)
}

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
```
You'll notice that the Rust compiler will probably complain it cannot convert our structs (e.g. CreateTodoItemRequest > TodoEntity). Hence, we need to add some conversion logic to our project.
To add conversion logic to our project, we should implement the [From trait](https://doc.rust-lang.org/std/convert/trait.From.html).
> You should always prefer using `From` over `Into` because implementing From automatically provides one with an implementation of Into thanks to the blanket implementation in the standard library.


In `todo_entity.rs` add the following implementations:

#### **`todo_api/src/entities/todo_entity.rs`**
```rust
// Convert from TodoEntity to TodoItem
impl From<TodoEntity> for TodoItem {
    fn from(entity: TodoEntity) -> Self {
        TodoItem {
            id: entity.id,
            title: entity.title,
            description: entity.description,
            completed: entity.completed,
            completed_at: entity.completed_at,
            created_at: entity.created_at,
        }
    }
}

// Convert from CreateTodoItemRequest to TodoEntity
impl From<CreateTodoItemRequest> for TodoEntity {
    fn from(request: CreateTodoItemRequest) -> Self {
        TodoEntity {
            id: Uuid::new_v4(),
            title: request.title,
            description: request.description,
            created_at: SystemTime::now(),
            completed_at: None,
            completed: false,
        }
    }
}

// Convert from UpdateTodoItemRequest to TodoEntity
impl From<UpdateTodoItemRequest> for TodoEntity {
    fn from(request: UpdateTodoItemRequest) -> Self {
        TodoEntity {
            id: Uuid::new_v4(),
            title: request.new_title,
            description: request.new_description,
            created_at: SystemTime::now(),
            completed_at: match request.completed {
                true => Some(SystemTime::now()),
                _ => None,
            },
            completed: request.completed,
        }
    }
}
```

## Testing our API

We can now manage todo items using Restful APIs. Let's try and add a todo item using a post method:
```shell
curl --header "Content-Type: application/json"  --request POST  --data '{"title":"Full stack","description":"Going full stack on rust"}' http://localhost:8080/todo -s | jq
```
results in :
```json
{
  "id": "80b94a3d-6b3f-4a57-a31d-9ef027ac8874",
  "title": "Full stack",
  "description": "Going full stack on rust",
  "completed": false,
  "completed_at": null,
  "created_at": {
    "secs_since_epoch": 1664196727,
    "nanos_since_epoch": 374136000
  }
}
```

We can now retrieve our new record  if we'd like:
```shell
curl http://localhost:8080/todo/80b94a3d-6b3f-4a57-a31d-9ef027ac8874 -s | jq
```

```json
{
  "id": "80b94a3d-6b3f-4a57-a31d-9ef027ac8874",
  "title": "Full stack",
  "description": "Going full stack on rust",
  "completed": false,
  "completed_at": null,
  "created_at": {
    "secs_since_epoch": 1664196727,
    "nanos_since_epoch": 374136000
  }
}
```

We can also update the record, setting a new title, description, and/or marking the item as completed:
```shell
curl --header "Content-Type: application/json" --request PUT --data '{"new_title":"Complete full stack on rust","new_description":"Going full stack on rust updated", "completed": true}' http://localhost:8080/todo/80b94a3d-6b3f-4a57-a31d-9ef027ac8874 -s | jq
```

```json
{
  "id": "80b94a3d-6b3f-4a57-a31d-9ef027ac8874",
  "title": "Complete full stack on rust",
  "description": "Going full stack on rust updated",
  "completed": true,
  "completed_at": {
    "secs_since_epoch": 1664197123,
    "nanos_since_epoch": 309961000
  },
  "created_at": {
    "secs_since_epoch": 1664196727,
    "nanos_since_epoch": 374136000
  } 
}
```

Finally, let's delete our record and see if we can retrieve it again:
```shell
curl -X DELETE http://localhost:8080/todo/80b94a3d-6b3f-4a57-a31d-9ef027ac8874
```
followed by 
```shell
curl http://localhost:8080/todo/80b94a3d-6b3f-4a57-a31d-9ef027ac8874 -s | jq
```
If our logic works fine, we should no longer get a result from the last request.

## Automatically apply pending Migrations
Whenever we make changes to our database model, we have to make sure migrations are applied to our databases. I mention plural here because you have one on your machine, and your co-worker has one too. Then there is a CD/CI chain for various environments (DEV/STAGING/PRODUCTION). Chances of us forgetting to apply these migrations are definitely there (unless managed by CD).

Diesel is capable of automatically applying changes to our database when our executable starts.

Let's add the migration appliance logic t our data module:

#### **`todo_api/src/data/mod.rs`**
```rust
use crate::Error;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.

    let mut connection = db_context::get_pool().get().unwrap();
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
```

and trigger this method from our main.rs file;

#### **`todo_api/src/data/mod.rs`**
```rust
// Add error and info logging macro usings here.
use log::{error, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Apply any ending database migrations upon startup of our application.
    match data::run_migrations() {
        Ok(()) => info!("Succesfully applied pending migrations (if any)"),
        Err(_) => error!("Unable to apply pending migrations"),
    }
    ... other startup logic ...
}
```

That's it, this wraps up our orm tooling chapter. It's been a long ride, but our backend project is slowly coming towards an end almost.

## BONUS IF YOU MADE IT HERE
If you're still wondering why we added the abstraction for `Repository<TodoEntity>`. I've added unit tests to the [todo controller](todo_api/src/api/todo_controller.rs). 
You'll see I've added a mock repository that mimics the behavior of our database variant of the repository but uses an in-memory datastore for it.
I can register that, instead of the original variant, and unit test (not integration test) my request handlers and mapping, without modifying any code. This is only possible because we rely on an abstraction.
