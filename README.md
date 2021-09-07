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

# Set Rust version to nightly
rustup default nightly 

# Build and run application
./scripts/init_db.sh # Starts and migrates a Postgres database using Docker
cargo run # Compiles and runs the Hermod project (aka cargo r)

```
Additional commands useful for developing in the api project are located
in the [api folder's README](./api)

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