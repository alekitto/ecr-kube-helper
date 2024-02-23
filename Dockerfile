FROM public.ecr.aws/docker/library/rust:1.76.0-bullseye as build-env

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ARG TARGETPLATFORM
WORKDIR /app
COPY . /app

RUN --mount=type=cache,id=registry-$TARGETPLATFORM,target=/usr/local/cargo/registry \
    --mount=type=cache,id=target-$TARGETPLATFORM,target=/app/target \
    cargo build --release && \
    cp /app/target/release/ecr-kube-helper /

FROM gcr.io/distroless/cc

COPY --from=build-env /ecr-kube-helper /
COPY --from=build-env /etc/ssl/certs /etc/ssl/certs
ENTRYPOINT ["/ecr-kube-helper"]
