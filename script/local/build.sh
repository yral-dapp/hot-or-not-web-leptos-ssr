cargo leptos build --release

docker stop hot-or-not-web-leptos-ssr
docker rm hot-or-not-web-leptos-ssr

docker build . --tag hot-or-not-web-leptos-ssr

docker run --detach --publish 8080:8080 --name hot-or-not-web-leptos-ssr hot-or-not-web-leptos-ssr