#!/bin/sh -e

export \
	SPN_SUPERPOSITION_URL=${SPN_SUPERPOSITION_URL:-https://testnet-rpc.superposition.so}

if [ -z "$SPN_SUPERPOSITION_KEY" ]; then
	>&2 echo "SPN_SUPERPOSITION_KEY unset"
	exit 1
fi

cargo stylus deploy \
	--no-verify \
	--endpoint "$SPN_SUPERPOSITION_URL" \
	--wasm-file priories.wasm \
	--private-key "$SPN_SUPERPOSITION_KEY"
