export LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl
export LEPTOS_HASH_FILES=true

cargo leptos build --release --lib-features release-lib --bin-features release-bin
