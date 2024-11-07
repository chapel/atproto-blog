set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

build:
    cargo build --release --bin client --no-default-features --target wasm32-unknown-unknown --features hydrate
    cargo build --release --bin server --no-default-features --target wasm32-unknown-unknown --features ssr
    wasm-bindgen target/wasm32-unknown-unknown/release/server.wasm --out-name index --no-typescript --target bundler --out-dir site
    wasm-bindgen target/wasm32-unknown-unknown/release/client.wasm --out-name index --no-typescript --target web --out-dir site/pkg

wrangler-dev:
    npx wrangler pages dev site

watch-build:
    watchexec -n -e rs just build

tailwind-watch:
    tailwindcss -i styles.css -o site/pkg/styles.css --watch

dev:
    deno run -A npm:concurrently "just watch-build" "just tailwind-watch" "just wrangler-dev"
