FROM scratch

WORKDIR /app

COPY ./target/x86_64-unknown-linux-musl/release/hash.txt .
COPY ./target/x86_64-unknown-linux-musl/release/hot-or-not-web-leptos-ssr .

COPY ./target/site ./site
ENV LEPTOS_SITE_ROOT="site"

ENV LEPTOS_ENV="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_HASH_FILES="true"
EXPOSE 8080

CMD ["./hot-or-not-web-leptos-ssr"]
