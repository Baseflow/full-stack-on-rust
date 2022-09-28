# Rest API

Our backends are usually RESTful API's containing a number of functional features we would like to embed in this project.

* [ ] http protocol handling
* [ ] asynchronous request handling
* [ ] implement REST api specification (GET, POST, PUT, DELETE)
* [ ] json serialization
* [ ] orm tooling for connecting to the database
* [ ] open api v3 spec / including swaggerui.
* [ ] Containerizing our API

## Pick your weapon
Let's start with building a webserver first that is able to run on a particular portnumber and handle Http requests for us.
There are a number of crates that can help us with this:

* ActixWeb
* Hyper
* Rocket
* Warp 
* Axum
* Iron
* Poem
* Tide
* Rouille

For this example, we'll be using the ActixWeb framework as it is very well known, very popular, well maintained, and supports our basic needs out of the box.
> I've read the book ['Hands on microservices' by Denis Kodolin](https://www.amazon.com/Hands-Microservices-Rust-scalable-microservices/dp/1789342759) which starts of using the bare minimum, just Hyper for the HttpServer. The developer experience is not as good as using frameworks like Actix-Web and Rocket, but is does explain what goes on under the hood, as all of these frameworks are based on Hyper. Highly recommended read if you want to dive deep into this.

## Adding actix web to our project
First, let's start off with adding actix-web to our `Cargo.Toml` file in the `todo_api` project:
#### **`todo_api/Cargo.toml`**
```toml
[dependencies]
actix-web = "4"
```

## Hello world
Setting up a simple webserver is pretty straight forward:

#### **`todo_api/src/main.rs`**
```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
```

We'll start by adding including all the required modules from the actix-web crate (`get`, `web`, `App`, `HttpServer` and `Responder`).

Our main function is setup to be our Tokio asynchronous entry point.
```rust
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
```

We'll define a new WebServer (`HttpServer::new()`), and bind it to portnumber 8080 on any address (`.bind((Ipv4Addr::UNSPECIFIED, 8080))`).

There are a couple of things to notice here:
There are 2 ways to register a route and a handler with actix-web.
* `.route("{PATH}", web::{METHOD}.To({HANDLER}))`
  * In this example: `.route("/hello", web::get().to(|| async { "Hello World!" }))`
* Adding the `#[{METHOD("{PATH}")}]` macro to a function, and registering this as `.service({FN_NAME})`.
  * In this example: 
  ```rust 
  #[get("/hello/{name}")]
  async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
  }
  ```
  combined with `.service(greet)`

Personally, I prefer the latter as this scales better when our project grows. It gives space to split endpoints to seperate handlers, across multiple files, tigh them in one place with the end point, and just to the registration of the handlers and router in our bootstrapper.

Note: the is also the option to combine these to methods, which might be convenient when you don't want to use the macro:
```rust
.route("/hello/{name}", web::get().to(greet))

async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
```

## Testing our webserver
We can now perform 2 http requests.
* `/hello` should return "Hello World!";
* `/hello/thomas` should return "Hello Thomas"

```shell
~ curl http://localhost:8080/hello
Hello World!
~ curl http://localhost:8080/hello/Thomas
Hello Thomas
```

