ARG RUST_VERSION=1.95.0
ARG APP_NAME=site

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git nodejs npm

COPY . .

RUN cd /app/frontend && \
    npm install && \
    npm run build && \
    mv /app/frontend/dist /app/dist

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp -r ./target/release/$APP_NAME /bin/server


FROM alpine:3.22 AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

COPY --from=build /app/dist /app/dist

WORKDIR /app

EXPOSE 8080

CMD ["/bin/server"]
