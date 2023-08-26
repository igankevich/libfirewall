#!/bin/sh
set -ex
pre-commit run --all-files --show-diff-on-failure
