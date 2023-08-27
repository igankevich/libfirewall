FROM rust:1.72

LABEL org.opencontainers.image.source=https://github.com/igankevich/libfirewall
LABEL org.opencontainers.image.description="CI image"

RUN apt-get -qq update \
    && apt-get -qq install --no-install-recommends \
    pre-commit \
    shellcheck \
    && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    RUST_VERSION=1.72 \
    PATH=/usr/local/cargo/bin:$PATH

RUN rustup toolchain add nightly --target x86_64-unknown-linux-gnu
RUN rustup toolchain add 1.72 \
    --target x86_64-unknown-linux-gnu \
    --component clippy rustfmt
RUN rustup default 1.72-x86_64-unknown-linux-gnu

RUN curl --location --silent --fail --output /usr/bin/shfmt https://github.com/mvdan/sh/releases/download/v3.7.0/shfmt_v3.7.0_linux_amd64 && \
    chmod +x /usr/bin/shfmt

CMD ["/bin/bash"]
