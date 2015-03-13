#!/bin/sh
cargo clean > /dev/null
cargo build > /dev/null
target/reaktor_fast_track_2 | jsonpp
