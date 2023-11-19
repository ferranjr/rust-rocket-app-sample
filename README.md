# Rust Playground
   
    WORK IN PROGRESS

## What's in here?
Minimal set up to experiment with Rust development by building an API.

Current Stack used:
* Rocket as Web Framework (https://rocket.rs/)
* Mongo DB as persistence layer (https://www.mongodb.com/docs/drivers/rust/current/)
* Goose for Load Testing (https://book.goose.rs/)
* Prometheus for metrics (https://prometheus.io/)
* Grafana to set up dashboard for observability of the the API (https://grafana.com/)
* Docker to pack our app (https://www.docker.com/)
* Docker compose to easy build and start the app (https://docs.docker.com/compose/)

## Code Structure
### App
This is the actual users API module, within it the code is structured using mods as follows:
* *api*: our API routes are defined here
* *models*: our models are defined in here, currently only user struct
* *repository*: user repository, mongo repo defined here

## Running The Code
In order to play with the user APIs, there is a docker-compose file to ease the usage:
```shell
docker compose up
```
This should spin up:
* **mongoDb**
* **cadvisor**
* **prometheus**
* **grafana**
* **users-api**: our rocket project, which will be build using a builder image + the the runner image to pack it

At the moment to run the performance test you will need to run them from your IDEA or CLI and
you need RUST set up for that. Check Goose book to see how to run them

## Goals
* [X] Minimal CRUD Api with persistence using Rust
* [X] Performance Test Set Up (bare minimum done, needs more work)
* [X] Easy To Run Project (docker compose up)
* [X] Add the postman collections
* [ ] Unit Test coverage
* [ ] API Documentation (Open API)
* [ ] Add basic auth/apikey
* [ ] Second API Server to interact between services
* [ ] Easy To Run Performance tests
* [ ] Export Mongo metrics into prometheus: mongo_exporter
* [ ] Export Docker metrics intro prometheus: cadvisor
* [ ] Add grafana dashboards to the repo

## Sources used
In order to build this, a part from the documentation of the documents, gone through
the following blogs:

* [Build a REST API with Rust and MongoDB - Rocket Version](https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5)
  * This article is most of the code of the app, started from scratch but found this article and used it to rewrite it in a more structured way
  * The article is great and should be the reference source to building a similar app for newbies like me
* [The Goose Book](https://book.goose.rs/example/simple.html)
* [Grafana Hisotgram How To](https://opstrace.com/blog/grafana-histogram-howto)
* [Have You Been Using Histrograms Metrics Correctly](https://medium.com/mercari-engineering/have-you-been-using-histogram-metrics-correctly-730c9547a7a9)

