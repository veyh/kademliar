#!/bin/bash
set -eo pipefail
IFS=$'\n\t'

script_dir=$(dirname "$(realpath "$0")")
cd "${script_dir}"

args=(
  --id 35a35935f5226f7a6adcb84aa4da1b62c71023e1
  --concurrency 1

  --bind 127.42.0.0:4200

  -b 127.42.0.0:4201
  -b 127.42.0.0:4202
  # -b 127.42.0.0:4203
  # -b 127.42.0.0:4204
  # -b 127.42.0.0:4205
)

exec cargo run -- "${args[@]}"

