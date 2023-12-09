#!/bin/sh
set -ex

cargo check --all-targets
cargo test --release --all-targets
cargo fmt -- --check
cargo clippy --all-targets -- --deny=warnings
