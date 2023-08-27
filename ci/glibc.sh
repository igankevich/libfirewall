#!/bin/sh
glibc_version="$(getconf GNU_LIBC_VERSION | sed -r 's/glibc (.*)/\1/')"
printf "glibc_version=%s" "$glibc_version" >>"$GITHUB_OUTPUT"
