#!/bin/sh
set -ex

cargo fmt -- --check
cargo clippy --all-targets -- --deny=warnings
cargo test --release --all-targets
