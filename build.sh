#!/bin/bash
cargo build --release --features generate-api-description --target=wasm32-unknown-unknown
wasm-build target {{project-name}} --target-runtime=substrate --final={{project-name}} --save-raw=./target/{{project-name}}-deployed.wasm --target wasm32-unknown-unknown
