#!/bin/sh

set -ex
apt-get -qq update
apt-get -qq install --no-install-recommends jq
rustup default 1.72-x86_64-unknown-linux-gnu
