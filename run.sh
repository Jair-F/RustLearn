#!/bin/bash

cargo check
cargo build

clear

cargo run --release --quiet