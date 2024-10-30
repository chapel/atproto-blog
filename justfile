set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

build:
    cargo build --release --bin client --no-default-features --target wasm32-unknown-unknown --features hydrate
    cargo build --release --bin server --no-default-features --target wasm32-unknown-unknown --features ssr
    wasm-bindgen target/wasm32-unknown-unknown/release/server.wasm --out-name index --no-typescript --target bundler --out-dir site
    wasm-bindgen target/wasm32-unknown-unknown/release/client.wasm --out-name index --no-typescript --target web --out-dir site/pkg

dev: build
    npx wrangler pages dev site

watch:
    watchexec -r -e rs just dev
