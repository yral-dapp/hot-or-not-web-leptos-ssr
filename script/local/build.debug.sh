cargo build --package=hot-or-not-web-leptos-ssr --lib --target=wasm32-unknown-unknown --no-default-features --features=hydrate

cargo build --package=hot-or-not-web-leptos-ssr --bin=hot-or-not-web-leptos-ssr --no-default-features --features=ssr

# docker rm hot-or-not-web-leptos-ssr

# docker build . --tag hot-or-not-web-leptos-ssr