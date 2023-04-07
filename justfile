# List target for this
@default:
  just --list

# Build & Run the current code
@run:
  cargo run

# Build the curren code
@build:
  cargo build

# Run tests
@test:
  cargo test