#!/bin/sh
set -ex
rust_flags="-Ccodegen-units=1 -Cstrip=symbols -Copt-level=3 -Cincremental=false -Clto=yes -Cembed-bitcode=yes"
target=x86_64-unknown-linux-gnu
env RUSTFLAGS="$rust_flags" \
    cargo build \
    --quiet \
    --release \
    --target "$target" \
    --no-default-features
. /etc/os-release
outfile="binaries/libfirewall-$ID-$VERSION_ID.so"
mkdir -p binaries
mv target/"$target"/release/libfirewall.so "$outfile"
