FROM debian:bookworm-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Download then execute (not piped) so curl failures stop the build.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh \
    && sh /tmp/rustup.sh -y --default-toolchain stable --profile minimal \
    && rm /tmp/rustup.sh
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /build/target/release/claude-lint /usr/local/bin/claude-lint
WORKDIR /workspace
ENTRYPOINT ["claude-lint"]
