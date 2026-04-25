#!/bin/bash
set -euo pipefail

wget --quiet https://pve.proxmox.com/pve-docs/api-viewer/apidoc.js -O - | sed '0,/const apiSchema = \[/s//[/' | sed -n '/^\[$/,/^\]$/p' > PVE-schema.json

version=$(wget https://pve.proxmox.com/pve-docs/index.html --quiet -O - | xmllint --html --recover --xpath 'string(//*[@id="revnumber"])' - 2>/dev/null | grep '([0-9]\.)+[0-9]' --extended-regexp --only-matching)
echo "You updated the schema to version $version"
