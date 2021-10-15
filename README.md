<div align="center">
  <img src="https://user-images.githubusercontent.com/5386772/137525840-d6703c94-f7d8-4e6a-9435-27380c923dff.png" width="30%"/>
  <h1>Hermod API</h1>
 <em>
  A platform for instant and seamless customer interaction. 
 </em>
</div>
<br />

<div align="center" markdown="1">
<a href ="https://deps.rs/repo/github/hermodapp/api" target="_blank"><img src="https://deps.rs/repo/github/hermodapp/api/status.svg" /></a>
<a href ="https://github.com/hermodapp/api/actions/workflows/general.yml"  target="_blank"><img src="https://github.com/hermodapp/api/actions/workflows/general.yml/badge.svg" /></a>
<a href="https://hermodapp.github.io/rustdocs/hermodapi/"  target="_blank">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

# Running API
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
./scripts/init_collector.sh # Starts an Open Telemetry collector using Docker
cargo +nightly run # Compiles and runs the Hermod project using an edge Rust build (aka cargo r)

```

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
cargo sqlx prepare -- -lib # Rebuild sqlx's cache used for compile-time SQL guarantees
cargo sqlx prepare --check -- --lib

docker build -t hermod_api . # Build the release image of the application (will take a *very* long time, Rust has infamously long release compilation times)
docker run -p 8000:8000 hermod_api # Run the release image of the application

cloc configuration src tests migrations scripts
```


# Project Architecture
- configurations contains three files - base.yaml, local.yaml, and production.yaml. Base.yaml contains default configuration shared between local and production, and local and production specify configuration settings that differ between the two environments.
- migrations
- scripts
- src
- tests
- .env
- Cargo.toml
- Dockerfile
- sqlx-data.json
