#!/bin/sh
set -ex
apt-get update -qq
apt-get -qq install --no-install-recommends curl ca-certificates
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh
sh /tmp/rustup.sh --help
sh /tmp/rustup.sh -y
rustup default 1.72-x86_64-unknown-linux-gnu
