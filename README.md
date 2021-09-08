# Hermod
The Hermod project is currently a monorepo split into two projects - api and www.

Api contains a Rust server application executable that serves as our project's backend.

Www contains a React webserver that serves as our project's frontend.

Spec.yaml contains Digital Ocean Application specifications for our project's deployment.

## API
### Running API
#### Dependencies
- [Rustup](https://rustup.rs)
- [psql](https://www.postgresql.org/download/)
- [Docker](https://www.docker.com/get-started)
- [sqlx (optional)](https://lib.rs/crates/sqlx-cli)
- [Bunyan (optional)](https://lib.rs/crates/bunyan)

#### Installation Instructions for Mac/Linux
```bash
cd api

# Install Rust, psql, and Docker
brew install rustup postgres # Install Rustup and psql command line tool 
brew cask install docker # Install Docker

# Build and run application
./scripts/init_db.sh # Starts and migrates a Postgres database using Docker
cargo +nightly run # Compiles and runs the Hermod project using an edge Rust build (aka cargo r)

```
Additional commands useful for developing in the api project are located
in the [api folder's README](./api)

# Hermod api
<!-- test -->

For instructions to run the project, look at the [repo-level README](https://github.com/cs495wifly/hermod).

## Rust project documentation
To view the project's auto-generated documentation, run `cargo doc --open` locally, or view 
[the latest version online](https://cs495wifly.github.io/hermod/docs/hermod).

## Useful commands
```bash
# Install optional Rust command-line utilities
cargo install sqlx-cli # (Optionally) Install sqlx CLI
cargo install bunyan # (Optionally) install Bunyan log formatter

# Other useful commands
cargo doc --open # Compiles and opens project documentation (aka cargo d)
cargo test # Runs unit and integration tests (aka cargo t)

cargo r | bunyan # Compiles and runs the project, piping log output to the Bunyan formatter
TEST_LOG=true cargo t | bunyan # Runs tests with logging, piping output to Bunyan

./scripts/stop_db.sh # Stops the PostgresDB Docker container

sqlx mig add YOUR_MIGRATION_NAME # Create a new sqlx migration
sqlx mig run # Run your new migration
cargo sqlx prepare -- --bin hermod # Rebuild sqlx's cache used for compile-time SQL guarantees

docker build -t hermod_api . # Build the release image of the application (will take a *very* long time, Rust has infamously long release compilation times)
docker run -p 8000:8000 hermod_api # Run the release image of the application
```


## Project Architecture
- configurations contains three files - base.yaml, local.yaml, and production.yaml. Base.yaml contains default configuration shared between local and production, and local and production specify configuration settings that differ between the two environments.
- migrations
- scripts
- src
- tests
- .env
- Cargo.toml
- Dockerfile
- sqlx-data.json
