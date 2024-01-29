#!/bin/bash
# cargo install wasm-pack before this
wasm-pack build --target no-modules --bin majestic-wasm
cp pkg/majestic_bg.wasm pkg/majestic.js ./misc/web/

