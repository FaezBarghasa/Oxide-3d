default:
    @just --list

check:
    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings

test:
    cargo test --workspace

fix:
    cargo fmt
    cargo clippy --fix --workspace --all-targets --allow-dirty

bench:
    cargo bench --workspace

deny:
    cargo deny check

audit:
    cargo audit

desktop:
    cargo run -p oxide-desktop

headless *ARGS:
    cargo run -p oxide-headless -- {{ARGS}}

server:
    cargo run -p oxide-server
