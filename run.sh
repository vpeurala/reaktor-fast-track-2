#!/bin/sh
cargo clean
cargo build
target/reaktor_fast_track_2 | jsonpp
