#!/bin/sh
set -ex

cargo check --all-targets
cargo fmt -- --check
