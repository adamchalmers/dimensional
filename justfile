@ci:
    just lint
    just test
    just fmt-check

lint:
    cargo clippy --tests --examples --benches

lint-fix:
    cargo clippy --tests --examples --benches --fix

test:
    cargo nextest run
    cargo test --doc

fmt-check:
    cargo fmt --check
    cargo sort --check
    typos
