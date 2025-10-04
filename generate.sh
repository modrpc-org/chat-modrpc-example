#!/bin/sh

set -e

# Change to this script's directory
cd $(dirname -- "$( readlink -f -- "$0"; )")

if [ ! -f .modrpc/std.modrpc ]; then
    mkdir -p .modrpc
    curl https://raw.githubusercontent.com/modrpc-org/modrpc/refs/heads/main/proto/std.modrpc -o .modrpc/std.modrpc
fi

modrpcc -l rust -n chat chat.modrpc
