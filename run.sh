#!/usr/bin/env bash
cargo clean
cargo build --release
time ./target/release/reaktor_fast_track_2

