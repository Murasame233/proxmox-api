#!/bin/sh

tmp=$(mktemp -d)
echo "Using temporary directory '$tmp'"

cargo run -- recursive ./PVE-schema.json "$tmp/generated.rs"

# Format the generated output
cargo fmt -- "$tmp/generated.rs"

# Clean destination folder to avoid "can't remove directory because non-empty"
# errors on cross-device moves (even with `mv -f`).
rm -r "../proxmox-api/src/generated"

mv "$tmp/generated.rs" "$tmp/generated/" "../proxmox-api/src/"

echo "Removing temporary directory '$tmp'"
rm -r "$tmp"