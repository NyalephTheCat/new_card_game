#!/usr/bin/env sh
set -euo pipefail
IFS=$'\n\t'

SERVER="eihwaz"
CLIENT="ansuz"

(trap 'kill 0' SIGINT; \
  bash -c "cd ./$CLIENT; trunk serve --proxy-backend=http://[::1]:8081/api/" & \
  bash -c "cd ./$SERVER; cargo watch -- cargo run --bin $SERVER -- --port 8081")