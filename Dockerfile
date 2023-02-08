# Note that the following build needs binaries to be precompiled for the target
# architectures. Use the `build-all` just recipies to build for all targets.
FROM alpine as arm-builder
COPY ./target/armv7-unknown-linux-musleabihf/release/manucure /manucure

FROM alpine as arm64-builder
COPY ./target/aarch64-unknown-linux-musl/release/manucure /manucure

FROM alpine as amd64-builder
COPY ./target/x86_64-unknown-linux-musl/release/manucure /manucure

FROM ${TARGETARCH}-builder AS builder

FROM alpine
MAINTAINER Paul Delafosse "paul.delafosse@protonmail.com"
USER manucure

# Install binaries
COPY --from=builder /manucure /usr/bin/manucure

# Install assets
COPY assets/ /opt/manucure/assets

EXPOSE 3000

COPY ./docker/entrypoint.sh /entrypoint.sh

CMD ["manucure"]
ENTRYPOINT ["/entrypoint.sh"]
