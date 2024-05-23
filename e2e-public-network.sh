#!/bin/bash
set -xeo pipefail
IFS=$'\n\t'

script_dir=$(dirname "$(realpath "$0")")
cd "${script_dir}"

cfg_bind_addr=$1

if [[ -z "${cfg_bind_addr}" ]]; then
  echo "usage: $0 <ipv4:port>"
  exit 1
fi

# I didn't add DNS support so we resolve the addresses here before passing to
# the app.

args=(
  --bind "${cfg_bind_addr}"
  -b "$(host router.bittorrent.com | head -n1 | awk '{print $NF}'):6881"
  -b "$(host router.utorrent.com | head -n1 | awk '{print $NF}'):6881"
  -b "$(host dht.transmissionbt.com | head -n1 | awk '{print $NF}'):6881"
)

exec cargo run -- "${args[@]}"

