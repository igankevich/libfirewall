#!/bin/sh
set -ex
git config --global --add safe.directory "$PWD"
git status
pre-commit run --all-files --show-diff-on-failure || cat .cache/pre-commit/pre-commit.log
