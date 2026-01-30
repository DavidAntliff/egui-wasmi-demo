default:
    just --list

ci:
    cargo clippy -- -D warnings
    cargo fmt --all -- --check

serve:
    @echo "http://127.0.0.1:8080/index.html#dev"
    trunk serve
