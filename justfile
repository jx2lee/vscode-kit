set positional-arguments

# Display help
help:
    just -l

# Run format check
fmt:
    cargo fmt -- --config imports_granularity=Item

# Build and run clippy
lint:
    cargo clippy --all-targets --all-features

build:
    cargo build --release