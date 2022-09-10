# Cargo workspaces

Workspaces can be used to split your project into multiple projects/artifacts.

In lots of projects, there is a often shared functionality across the entire project.

Rather then duplicating code, you can create a workspace with multiple projects and share functionality across that workspace.

## Example

Let's say we want to create a client-server application (a Todo app). There is an interface definition shared by the client and the server application.

Imagina the following `Cargo.toml` file.

```yaml
[workspace]
members = [
    "todo_api",
    "todo_frontend",
    "todo_app",
    "todo_shared"
]
```

We have a `todo_api` crate, which is responsible for serving the todo REST api's.<br/>
There is also a `todo_frontend` crate, which will be serving a WASM webapplication and communicate with the API to fetch it's data.
The same goes for the native Linux application `todo_app`, which will also be communicating with the API to fetch the same data.
In order to share models and perhaps logic between applications, we create another crate called `todo_shared` which will contain all shared models and logic for other projects in the workspace.

Let's say we have the following model for TodoItem defined in todo_shared > models > todo_item.rs
```rust
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct TodoItem<'a> {
    // The unique identifier of the todo item
    pub id: uuid::Uuid,

    // The title of the todo item
    pub title: &'a str,

    // The description of the todo item
    pub description: &'a str,

    // Indicates whether the todo item is completed
    pub completed: bool,

    // Epoch timestamp when the todo item was completed
    pub completed_at: SystemTime,

    // Epoch timestamp when the todo item was created
    pub created_at: SystemTime,
}

impl<'a> TodoItem<'a> {
    pub fn new(title: &'a str, description: &'a str) -> Self {
        TodoItem {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            completed_at: SystemTime::now(),
            created_at: SystemTime::now(),
        }
    }
}

```

In our todo_api project, we can use `todo_shared` and make use of the public members of that crate, which is TodoItem in this case and the `New` function
```rust
use todo_shared::models::todo_item::todoitem;
fn main() {
    let todo_item = todoitem::new("going full stack on rust", "let's go full stack on rust");
    println!("created a new todo item : {:?}", todo_item);
}
```
