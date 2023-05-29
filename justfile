## Build
migrate-db:
    cd crates/manucure-db
    docker-compose -f docker-compose.dev.yml up postgres -d
    sqlx migrate run
    cargo sqlx prepare --merged

# needed between cross build, otherwise some link to GLIC are broken
clean-targets:
    rm -rd target/release
    rm target/.rustc_info.json

build-x86: migrate-db
    CROSS_CONFIG=Cross.toml cross build --target x86_64-unknown-linux-musl --release
    just clean-targets

build-arm-v7: migrate-db
    CROSS_CONFIG=Cross.toml cross build --target armv7-unknown-linux-musleabihf --release
    just clean-targets

build-arm-64: migrate-db
    CROSS_CONFIG=Cross.toml cross build --target aarch64-unknown-linux-musl --release
    just clean-targets

build-all: build-x86 build-arm-v7 build-arm-64

docker-build: build-all
    docker buildx build --no-cache --push --platform linux/amd64,linux/arm/v7,linux/arm64/v8  . -t oknozor/manucure:latest

tailwind:
    tailwindcss -m -i assets/style.css -o assets/tailwind.min.css --watch
