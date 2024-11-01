set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

build:
    cargo build --release --bin client --no-default-features --target wasm32-unknown-unknown --features hydrate
    cargo build --release --bin server --no-default-features --target wasm32-unknown-unknown --features ssr
    wasm-bindgen target/wasm32-unknown-unknown/release/server.wasm --out-name index --no-typescript --target bundler --out-dir site
    wasm-bindgen target/wasm32-unknown-unknown/release/client.wasm --out-name index --no-typescript --target web --out-dir site/pkg

wrangler-dev:
    npx wrangler pages dev site

watch-build:
    watchexec -e rs just build

# Clean up any leftover files from previous runs
cleanup:
    pwsh -File scripts/cleanup.ps1

# Start the watch-build process in the background
start-watch:
    pwsh -File scripts/start-watch.ps1

# Clean up the watch-build process
stop-watch:
    pwsh -File scripts/stop-watch.ps1

# Main development command that composes the above
dev:
    just cleanup
    just start-watch
    try { just wrangler-dev } finally { just stop-watch }
