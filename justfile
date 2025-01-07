# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    cd  asset_server && just verify
    echo ------------ verify done! ------------  

kill port:
     lsof -t -i:{{port}} | xargs -r kill
    
files:
    just kill 12501
    cd asset_server && just run &
    
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
    cd asset_server && just fmt
    cargo fmt

publish:
    cargo publish --token $GLOBAL_CARGO_TOKEN

l_pub:
    cargo publish

m:
    html2maud-bin