#!/usr/bin/env fish

cargo +nightly fmt
cargo clippy
cargo test
cargo xtask docgen
bell
