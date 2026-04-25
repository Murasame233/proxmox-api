#!/bin/sh

cargo run -- recursive ./PVE-schema.json ../proxmox-api/src/generated.rs

# Format the generated output
cargo fmt --manifest-path ../proxmox-api/Cargo.toml