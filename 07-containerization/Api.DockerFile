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
