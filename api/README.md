# Hermod api

For instructions to run the project, look at the [repo-level README](../).

```
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