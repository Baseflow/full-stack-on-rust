# Rest API

Our backends are usually RESTful API's containing a number of functional features we would like to embed in this project.

* [x] http protocol handling
* [x] asynchronous request handling
* [x] implement REST api specification (GET, POST, PUT, DELETE)
* [x] json serialization
* [x] orm tooling for connecting to the database
* [x] open api v3 spec / including swaggerui.
* [ ] Containerizing our API

We already covered the first 6 requirements, let's see if can wrap this up in a nice container so we can deploy this wherever we like.

## Choosing the right image:
First of, we'll need to create a [Docker file](todo_api/Dockerfile) to be able to build our container image.
There are a number of base images to take here:
- FROM scratch (Very small)
- FROM alpine (Small)
- FROM gcr.io/distroless/cc (Larger)
- FROM buster-slim (Large)

Of course, we're aiming for the smallest image for a number of reasons. 
- There are less vulnerabilities to be found in minimal distro's. 
- Users can pull images quicker, and they don't consume local storage as much.
- Images take up less space on your docker repository. In case you have to pay for storage, this might be convenient.
- Less bandwith = less costs
- Less risk of dependency conflicts

But of course, using smaller base-images also come with some trade offs. For example. OpenSSL (required to communicate with Postgresql) is not available on the scrath and alpine image out of the box. We should include it statically link it to our executable. This seems like a lot of hassle, and luckily, there is a more simple solution available. We can just include the OpenSSL crate to our api project and make sure it is available in our artifact.

Go ahead and add OpenSSL to our **Cargo.toml** file:

#### **`todo_api/cargo.toml`**
```toml
# needed for postgres with musl builds.
openssl = "*"
```

We will also need to include the openssl crate to our todo_api.

#### **`todo_api/src/main.rs`**
```rust
// Needed for musl builds.
extern crate openssl;
```
We can now create our docker file.
> Note that we create a stripped down version of our workspace yaml file on the fly. We don't need the APP and the Frontend projects here.

#### **`Api.DockerFile`**
```Dockerfile
FROM clux/muslrust
RUN mkdir /source
WORKDIR /source

RUN echo '[workspace]\nmembers = [\n\t"todo_shared",\n\t"todo_api",\n]' > ./Cargo.toml
COPY ./todo_api/Cargo.toml ./todo_api/Cargo.toml
COPY ./todo_api/src/ ./todo_api/src/
COPY ./todo_api/migrations/ ./todo_api/migrations/
COPY ./todo_shared/Cargo.toml ./todo_shared/Cargo.toml
COPY ./todo_shared/src/ ./todo_shared/src/
RUN cargo build --release --bin todo_api
RUN strip ./target/x86_64-unknown-linux-musl/release/todo_api

FROM scratch
COPY --from=0 /source/target/x86_64-unknown-linux-musl/release/todo_api /
CMD ["./todo_api"]
```
We can test if this Dockerfile builds by running
```shell
docker build -f Api.DockerFile . -t todo_api:local
```

What would be even cooler, if we could just spin up our api server, with a postgresql database in one go.
#### **`docker-compose.yaml`**
```yaml
version: "3.9"
networks:
  default:
    name: todo_api
services:
  db:
    image: "postgres"
    restart: always
    environment:
      POSTGRES_USER: todo_api_rw
      POSTGRES_PASSWORD: hello_rust
      POSTGRES_DB: todo_api
    healthcheck:
      test: [ "CMD-SHELL", "pg_isready -U todo_api_rw -d todo_api" ]
      interval: 5s
      timeout: 5s
      retries: 5
  todo_api:
    depends_on:
      db:
        condition: service_healthy
    image: todo-api-rust:local
    container_name: todo_api_rust_local 
    build:
      context: .
      dockerfile: Api.DockerFile
    ports:
      - 8080:8080
    environment:
      - DATABASE_URL=postgres://todo_api_rw:hello_rust@db/todo_api
      - RUST_LOG=debug #optional
      - RUST_BACKTRACE=1 #optional
```
You can start this orchestration by running
```shell
docker-compose up
```

NOW, what is really cool is that on startup, all our migrations are automatically applied as we implemented by the end of chapter **05-orm**. This means what we don't need to worry about setting up the database. We just spin it up, and are ready to go. 

Running `docker-compose up` immidiatly gives me a working environment to continue our future endeavors **going Full Stack on Rust**.