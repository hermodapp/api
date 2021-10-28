# cargo install just
# just manual: https://github.com/casey/just/#readme
set dotenv-load := true

_default:
    @just --list

# Start Docker containers
start:
	#!/usr/bin/env bash
	set -euxo pipefail
	cargo install sqlx-cli

	if [ ! $( docker ps | grep postgres | wc -l ) -gt 0 ]; then
		./scripts/init_db.sh
	fi
	
	if [ ! $( docker ps | grep otel | wc -l ) -gt 0 ]; then
		if [ -z ${HONEYCOMB_API_KEY+x} ]; then 
			echo "HONEYCOMB_API_KEY is not set. Your local server will not upload its tracing diagnostics to Honeycomb..."; 
		fi
		./scripts/init_collector.sh
	fi

# Stop Docker containers
stop:
	./scripts/stop_containers.sh

# Run application
run: start
	cargo run | bunyan

# Runs unit tests
test:
	ulimit -n 10000
	cargo test -q --locked

# Count lines of code
cloc:
	cloc configuration src tests migrations scripts

# Coverage analysis
coverage:
	cargo llvm-cov -q --open --ignore-filename-regex "build.rs|src\/main.rs"

# Format code and fix common anti-patterns
fix:
	cargo clippy --fix
	cargo fmt

# Run CI pipeline
ci: coverage
	cargo clippy --locked -- -D warnings
	cargo fmt -- --check

# Update sqlx-data.json
sqlx: 
	cargo sqlx prepare -- --lib