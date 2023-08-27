#!/bin/sh
set -ex
cargo test --quiet --no-run
cargo test --no-fail-fast -- --nocapture
