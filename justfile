# Generate ts bindings
bindings:
  cd src-tauri && cargo test

# Setup dev environment
dev:
  cargo tauri dev

# Build MacOS bundle
build:
  cargo tauri build --target aarch64-apple-darwin

# Starts Mock Server at localhost:3005
mock:
  cd mock_server && node server.js

# Generate icons from ./app-icon.png
icon:
  cargo tauri icon

# Run script to generate psl from hosts mentioned in recipe-scrapers library
get_valid_hosts:
  cd src-tauri/scripts/get_valid_hosts && cargo run

# Get line count of typescript project files
ts_lc:
  @bash -c "find ./components ./pages \( -name '*.tsx' -o -name '*.ts' \) | xargs wc -l"

# Get line count of rust project files
rs_lc:
  @bash -c "find ./src-tauri/src ./src-tauri/scripts/get_valid_hosts/src/ -name "*.rs" | xargs wc -l"

# Get line count for both typescript and rust project files
lc: ts_lc rs_lc
