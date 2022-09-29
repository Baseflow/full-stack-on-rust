# Rest API

Our backends are usually RESTful APIs containing several functional features we would like to embed in this project.

* [ ] HTTP protocol handling
* [ ] Asynchronous request handling
* [ ] Implement the REST api specification (GET, POST, PUT, DELETE)
* [ ] Json serialization
* [ ] ORM tooling for connecting to the database
* [ ] Open API V3 spec / including swagger-ui.
* [ ] Containerizing our API

## Pick your weapon
Let's start with building a web server first that can listen on a particular port number and handle HTTP requests for us.
Several crates can help us with this:

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
> I've read the book ['Hands on microservices' by Denis Kodolin](https://www.amazon.com/Hands-Microservices-Rust-scalable-microservices/dp/1789342759) which starts off using the bare minimum, just Hyper for the HttpServer. The developer experience is not as good as using frameworks like Actix-Web and Rocket but is does explain what goes on under the hood, as all of these frameworks are based on Hyper. Highly recommended to read if you want to dive deep into this.

## Adding actix web to our project
First, let's start with adding actix-web to our `Cargo.Toml` file in the `todo_api` project:
#### **`todo_api/Cargo.toml`**
```toml
[dependencies]
actix-web = "4"
```

## Hello world
Setting up a simple webserver is pretty straightforward:

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

We'll start by adding all the required modules from the actix-web crate (`get`, `web`, `App`, `HttpServer`, and `Responder`).

Our main function is set up to be our Tokio asynchronous entry point.
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

I prefer the latter as this scales better when our project grows. It gives space to split endpoints to separate handlers, across multiple files, thigh them in one place with the endpoint, and just to the registration of the handlers and router in our bootstrapper.

Note: the is also the option to combine these two methods, which might be convenient when you don't want to use the macro:
```rust
.route("/hello/{name}", web::get().to(greet))

async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
```

## Testing our webserver
We can now perform 2 HTTP requests.
* `/hello` should return "Hello World!";
* `/hello/thomas` should return "Hello Thomas"

```shell
~ curl http://localhost:8080/hello
Hello World!
~ curl http://localhost:8080/hello/Thomas
Hello Thomas
```

