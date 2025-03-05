FROM scratch

WORKDIR /app
COPY ./target/x86_64-unknown-linux-musl/release/hot-or-not-web-leptos-ssr ./x86_64-unknown-linux-musl/release/hot-or-not-web-leptos-ssr
COPY ./target/x86_64-unknown-linux-musl/release/hash.txt ./x86_64-unknown-linux-musl/release/hash.txt

COPY ./target/site ./site
ENV LEPTOS_SITE_ROOT="site"

ENV LEPTOS_ENV="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_HASH_FILES="true"
ENV LEPTOS_TAILWIND_VERSION="v3.4.17"
EXPOSE 8080

CMD ["./x86_64-unknown-linux-musl/release/hot-or-not-web-leptos-ssr"]
