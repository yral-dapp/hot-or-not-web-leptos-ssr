# Build the project in release mode
cargo leptos build --release

# Stop and remove the container if it exists
docker stop hot-or-not-web-leptos-ssr
docker rm hot-or-not-web-leptos-ssr

# Build the docker image
docker build . --tag hot-or-not-web-leptos-ssr --no-cache

# Run the docker image
docker run --detach --publish 8080:8080 --name hot-or-not-web-leptos-ssr hot-or-not-web-leptos-ssr