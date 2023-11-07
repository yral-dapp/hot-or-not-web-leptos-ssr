FROM rustlang/rust:nightly-bullseye

WORKDIR /app

COPY ./target/release/hot-or-not-web-leptos-ssr .
COPY ./target/site ./site
COPY Cargo.toml .

# Set any required env variables and
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

CMD ["./hot-or-not-web-leptos-ssr"]