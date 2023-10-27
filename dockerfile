FROM scratch
WORKDIR /app
COPY ./target/x86_64-unknown-linux-musl/release/hot-or-not-web-leptos-ssr .
CMD ["./hot-or-not-web-leptos-ssr"]