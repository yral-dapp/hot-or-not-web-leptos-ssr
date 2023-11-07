cargo leptos build --release

docker rm hot-or-not-web-leptos-ssr

docker build . --tag hot-or-not-web-leptos-ssr

docker run --detach --publish 3000:3000 --name hot-or-not-web-leptos-ssr hot-or-not-web-leptos-ssr