# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------  

run:
    cargo run

# Run tests    
test:
    cargo test

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt


publish:
    cargo publish --token $GLOBAL_CARGO_TOKEN

l_pub:
    cargo publish