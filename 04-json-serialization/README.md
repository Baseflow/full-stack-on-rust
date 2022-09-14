# Rest API

Our backends are usually RESTful API's containing a number of functional features we would like to embed in this project.

* [x] http protocol handling
* [x] asynchronous request handling
* [x] implement REST api specification (GET, POST, PUT, DELETE)
* [ ] json serialization
* [ ] orm tooling for connecting to the database
* [ ] open api v3 spec / including swaggerui.
* [ ] jwt token validation for incoming requests

We already covered the first 3 requirements, let's see if we can get some actual todo items over the wire.
To serialize to JSON and deserialize from JSON, we will need another crate, as it is not part of the `std` library.
There are a number of Crates that can do this for us, but the most well known crate is [Serde](https://docs.rs/serde_json/latest/serde_json/), so we'll stick with that.

Let's add Serde to our `Cargo.Toml` file. Note that our `todo_item` stuct resides in the `todo_shared` crate. So we will need to add it there. We will use this crate for both our backend as frontend projects. Any project including the `todo_shared` crate, will directly be able to (de)serialize `todo_item` entities from and to JSON..

Additionally, we'll add the `derive` feature from Serde, to be able to make use of the device macro and keep our code nice and clean.
#### **`todo_shared\Cargo.toml`**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

We can now add the Serialize and Deserialize derive macro's on top of our `todo_item` struct;

#### **`todo_shared\Cargo.toml`**
```rust
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem<'a> {
    ...
}
```
