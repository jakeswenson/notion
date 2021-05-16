#!/usr/bin/env bash

: "${FEATURE_BUILD:=default}"

CARGO_ARGS=()

function set_cargo_args() {
    if [ "$FEATURE_BUILD" != "default" ]; then
        CARGO_ARGS+=(--no-default-features)
    fi
}

set_cargo_args

set -x

cargo build --verbose --release "${CARGO_ARGS[@]}"
