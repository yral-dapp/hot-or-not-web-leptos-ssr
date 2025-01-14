rm -rf app/

export LEPTOS_HASH_FILES=true
LEPTOS_SITE_ROOT="target/site" cargo leptos build --bin-features local-bin --lib-features local-lib

mkdir -p app/
cp -rf target/site app/
cp -rf ssr/target/debug/hash.txt app/
cp target/debug/hot-or-not-web-leptos-ssr app/

cd app/

LEPTOS_SITE_ROOT="site" ./hot-or-not-web-leptos-ssr
