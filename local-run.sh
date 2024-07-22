#!/bin/bash

cargo leptos build --bin-features local-bin --lib-features local-lib
LEPTOS_SITE_ROOT="./target/site" LEPTOS_HASH_FILES=true ./target/debug/hot-or-not-web-leptos-ssr
