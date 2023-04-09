#!/bin/sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name labrat --out-dir wasm --target web target/wasm32-unknown-unknown/release/bevy_jam_3.wasm
basic-http-server wasm
