#!/bin/sh
wasm-pack build --release --target web --out-dir out
rm out/.gitignore
