#!/bin/sh
set -ex
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh
sh /tmp/rustup.sh
rustup default 1.72-x86_64-unknown-linux-gnu