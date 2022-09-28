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

#### **`todo_api/Cargo.toml`**
```toml
# Needed for Postgres with musl builds.
openssl = "*"
```

