# Rest API

Our backends are usually RESTful API's containing a number of functional features we would like to embed in this project.

* [x] http protocol handling
* [x] asynchronous request handling
* [x] implement REST api specification (GET, POST, PUT, DELETE)
* [x] json serialization
* [ ] orm tooling for connecting to the database
* [ ] open api v3 spec / including swaggerui.
* [ ] jwt token validation for incoming requests

We already covered the first 4 requirements, let's see if we can persist our data in a database.
For communication with the database, ORM tooling is usually the way we interact with our database.
ORM has a couple of advantages: it abstracts actual querying to the database for us. Whether we will be using Postsqres or SqLite, the specifics of that interfacing with a particular type of database will be handled by the ORM tooling. We are not bothered with it in our codebase. 
Additionally, ORM tooling will take care of serializing/deserializing object from and to the database. ORM tooling can also be used to keep our database up to date with migrations, which simplies devops. 

There are a couple of ORM crates available for Rust.
* Diesel
* Sea-orm
* Rustorm

Diesel is by far the most wel known ORM crate with over 4.5 million downloads. It supports SqLite, Postsqres and MySql out of the box, but can be extended with other database engines. We'll stick with that for our full-stack adventure.

Let's add diesel to our api project. We'll also add `dotenvy` to make use of environment variables. 
For connection pooling we'll use `r2d2`. Additionally, we'll also add `diesel_migrations` for automatic migration deployment upon application startup. 
Finally we'll add `env_logger` and `log` to add a logging framework.

#### **`todo_api/Cargo.toml`**
```toml
[dependencies]
diesel = { version = "2.0.0", features = ["postgres", "r2d2"] }
dotenv = "0.15.0"
diesel_migrations = "1.4.0"
r2d2 = "0.8.9"
env_logger = "0.9.0"
log = "0.4.17"
```

You should also install the `diesel_cli` binary to make use of the CLI tooling diesel offers, to create migrations and schema's.
In this case, we are only interested in interfacing with postgres, so don't compile tooling for other database engines just yet.


```shell
cargo install diesel_cli --no-default-features --features postgres
```

I've added a docker-compose file which hosts postgres on a non-default port. You can just run this to run postgres, and it will most likely not conflict with other postgres instances you might have running on your machine.

Navigate to the **todo_api/docker/postgres** folder and run 

```shell
docker-compose up -d && docker-compose logs -f
```
This will spin up a docker image with postgres on port 5555. It runs in a detached mode, to quitting the command using `ctrl + c` will not kill the container.

Now that our database is running, we can hook up diesel to it.
Let's start by setting some environment variables for Diesel to tell on which endpoint to connect with diesel, and what credentials to use.
Navigate to the **todo_api** directory and run the following command:

```shell
echo DATABASE_URL=postgres://full-stack:on-rust@localhost:5555/todo_api > .env
```

We can let diesel_cli create the database for us using `diesel setup`. This will also validate if Diesel is able to connect to the database using the DATABASE_URL from the environment variables.
Additionally, a migrations folder is already created with an 'Initial setup' migration.

Migrations always contain up and down scripts, used for applying and un-applying migrations as you want.

### Adding tables to the database
As I mentioned before, ORM tooling can be usefull to create/manage migrations, and have them applied to the database for us.
Diesel_cli can be used to create migrations.
```shell
diesel migration generate create_todo_table
```
This will create a new migration in the migrations folder.
In here we can specify what the required actions are for applying the migration, and undoing the migration.

For example: if we want to apply the migration, we'll need to create a todo table to the database.
If we want to rollback this migration, we want to delete the todo table.

Navigate to the newly created `create_todo_table` folder and add the following statement to the `up.sql` and `down.sql` file:

#### **`todo_api/migrations/2022-09-23-122632_create_todo_table/up.sql`**
```sql
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
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

## Query todo items.
To make sure our item can be queried, serialized and deserialized from/to the database, we'll need to add the `Queryable` derive macro to our todo_item.
However, the current todo_item model we have represent something that defines our public interface from/to the api and the clients. No clien should have to depend on diesel in order to function.
Therefore, we'll create a new todo_item entity, which represents the todo_api as returned/send to the database. Only our API project knows about the underlying datasource, and how to interact with that. We can use mapping later to map our entity to the public interface model, returned by our API to the clients.
You'll notice that it looks very similar, but it has different functions. Serde is not used here for JSON serialization as we don't require that here. However, we included the Queryable macro in order to interact with the database.

#### **`todo_api/src/entities/todo_item.rs`**
```rust
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct TodoItem {
    // The unique identifier of the todo item
    pub id: u32,

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
```
## Connection pooling.
Creating a database connection can easily be done with Diesel. However, creating a new connection for every incoming request doesn't scale all that well. Additionally, Postgresql only allows 100 connections to be opened by default to the server, and will, with high loads, eventually block new connections from being created.

In order to solve this, we'll be using connection pooling. We'll keep a maximum of let's say, 10 connections, alive and re-use them. This scales a lot better as the load on our server increases over time.

Lets create a **db_context.rs** file which handles all the connection pooling for us:
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
This part is more of a personal preference, but I usually use a repository pattern for my data layer interfacing. My controllers and business logic should not be aware on there the data is stored, but rather just know that there is 'a' place (repository) where todo items are stored. I should be able to interchange repositories implementations by changing registrations, rather then changing business logic all over the place. Again, this is personal, as an ORM already abstracts a lot of stuff for us. Don't follow up on my advice if you don't want to.

Let's create a repository pattern by defining the `Repository<T>` trait. It defines a generic astraction for basic CRUD actions on a data source.


