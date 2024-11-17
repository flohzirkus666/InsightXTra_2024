FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

# Create appuser
ENV USER=rust
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Copy our build
COPY --from=builder target/x86_64-unknown-linux-musl/release/rust-rest_endpoint ./rust_api

USER rust:rust

# Setting env vars
ENV ONTAP_HOST $ONTAP_HOST
ENV ONTAP_USERNAME $ONTAP_USERNAME
ENV ONTAP_PASSWORD $ONTAP_PASSWORD

CMD ["/rust_api"]