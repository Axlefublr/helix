#!/usr/bin/env fish

cargo +nightly fmt || return 1
cargo clippy || return 1
cargo test || return 1
cargo xtask docgen || return 1
printf \a
