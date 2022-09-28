# Full stack on Rust

This is the code and documentation repository for our 'Full stack on Rust' meetup on 29-9-2022.
It includes step by step documentation on how to setup a full rust stack for both backend as front end applications, along with source code.

We'll be creating a complete todo platform. Backend, front-end and desktop application, relying fully on Rust.
Here's our current list of requirements:
* Backend
  * [x] http protocol handling
  * [x] asynchronous request handling
  * [x] implement REST api specification (GET, POST, PUT, DELETE)
  * [x] json serialization
  * [x] orm tooling for connecting to the database
  * [x] open api v3 spec / including swaggerui.
  * [ ] Containerizing our API
* web-app
    * t.b.d.
* desktop-app
    * t.b.d.

1. [Workspaces](01-workspaces)
2. [Backend webserver](02-backend-webserver)
3. [Rest API](03-rest-api)
    1. [Implementing all Rest methods](03-rest-api/README.md#Implementing-all-Rest-methods)
    2. [Registering the controller methods](03-rest-api/README.md#Registering-the-controller-methods)
    3. [Testing our endpoints](03-rest-api/README.md#Testing-our-endpoints)
4. [JSON Serialization](04-json-serialization)
5. [ORM](05-orm)
    1. [Diesel](05-orm/README.md#diesel)
    2. [Adding tables to the database](05-orm/README.md#Adding-tables-to-the-database)
    3. [Query todo items](05-orm/README.md#Query-todo-items)
    4. [Connection pooling](05-orm/README.md#Connection-pooling)
    5. [Repository pattern](05-orm/README.md#Repository-pattern)
    6. [Using the repository](05-orm/README.md#Using-the-repository)
    7. [Testing our API](05-orm/README.md#Testing-our-API)
    8. [Automatically apply pending Migrations](05-orm/README.md#Automatically-apply-pending-Migrations)
6. [Open API Spec](06-open-api-spec)
    1. [Including our public models to the Open API V3 spec](06-open-api-spec/README.md#including-information-about-our-endpoints)
    2. [Including information about our endpoints](06-open-api-spec/README.md#including-information-about-our-endpoints)
    3. [Configure open api](06-open-api-spec/README.md#setup-swagger-ui)
7. [Containerization](07-containerization)
    1. [Choosing the right image](07-containerization/README.md#choosing-the-right-image)
    1. [Embedding openssl to our artifact](07-containerization/README.md#Embedding-openssl-to-our-artifact)
    1. [Creating the dockerfile](07-containerization/README.md#creating-the-dockerfile)
    1. [Composing it all together](07-containerization/README.md#composing-it-all-together)
    1. [Ready to go](07-containerization/README.md#ready-to-go)
