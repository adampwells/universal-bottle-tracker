# universal-bottle-tracker
App for tracking qr-coded beer bottles for home brewing

## Where can I use this?
This is offered as a free service at https://app.wht1.au

You can also build this project and host it yourself if you like.

## What is the tech stack?
This is a Rust project, using the Axum web framework and runs from a SQLite database. 

The Axum project is configured to serve some static files from a particular directory.
The contents of that directory is a Single Page Application [built using Quasar](https://quasar.dev).

Everything is containerised using Docker, and the `docker-compose.yml` file is used to start the app in Docker Swarm where we expect to have Traefik running..

There is a script called `buildPushDockerHub.sh` which will build the Docker image and push it to Docker Hub.