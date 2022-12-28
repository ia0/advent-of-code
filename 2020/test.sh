#!/bin/sh
set -ex

cargo fmt -- --check
cargo check --all-targets
