#!/bin/sh
set -ex
git status
pre-commit run --all-files --show-diff-on-failure || cat .cache/pre-commit/pre-commit.log
