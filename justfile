lint:
    cargo clippy --tests --examples --benches

test:
    cargo nextest run
    cargo test --doc
