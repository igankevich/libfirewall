#!/bin/sh

set -ex
apt-get -qq update
apt-get -qq install --no-install-recommends \
    pre-commit \
    shellcheck

rustup toolchain add nightly --target x86_64-unknown-linux-gnu
rustup toolchain add 1.72 \
    --target x86_64-unknown-linux-gnu \
    --component clippy rustfmt
rustup default 1.72-x86_64-unknown-linux-gnu

curl --location --silent --fail --output /usr/bin/shfmt https://github.com/mvdan/sh/releases/download/v3.7.0/shfmt_v3.7.0_linux_amd64
chmod +x /usr/bin/shfmt
