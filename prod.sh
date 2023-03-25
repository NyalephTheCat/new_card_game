#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

SERVER="eihwaz"
CLIENT="ansuz"

pushd $CLIENT
trunk build --release
popd

cargo run --bin $SERVER --release -- --port 8080 --static-dir ./dist