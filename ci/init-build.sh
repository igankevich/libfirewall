#!/bin/sh
# shellcheck disable=SC1091
set -ex
apt-get update -qq
apt-get -qq install --no-install-recommends curl ca-certificates gcc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh
target=x86_64-unknown-linux-gnu
toolchain=1.72-$target
sh /tmp/rustup.sh -y --default-toolchain $toolchain --target $target
. "$HOME/.cargo/env"
rustup default $toolchain
