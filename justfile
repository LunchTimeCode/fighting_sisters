# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------  

kill port:
     lsof -t -i:{{port}} | xargs -r kill
    
run: tw
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
    cargo fix --allow-dirty --allow-staged

w:
    cargo watch --ignore 'assets/css' -s 'just run'

publish:
    cargo publish --token $GLOBAL_CARGO_TOKEN

l_pub:
    cargo publish

m:
    html2maud-bin

tw-install:
    bun install -D tailwindcss
    bunx tailwindcss init

d-install:
    bun add -D daisyui@latest

tw:
     bunx tailwindcss build -i ./assets/css/styles.css -o ./assets/css/tw.css --minify
