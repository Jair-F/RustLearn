#!/bin/bash

pre-commit run --all-files

cargo clippy
cargo audit
cargo fmt -- --check
cargo fmt
