#!/bin/env bash

# Run a bunch of commands testing various combinations.
# 
# Note that the red zone is set to 4MiB to ensure that a segment is always
# created.

# DO NOT set -e because we want to run all commands even if one of them fails.

function run() {
    printf "\n"
    echo "*** Running with parameters: $*"
    cargo run -- "$@"
    echo "*** Exit code: $?"
}

run
run --use-stacker once
run --use-stacker always
run --use-stacker once --red-zone $((4 * 1024 * 1024))
run --new-thread
run --new-thread --use-stacker once
run --new-thread --use-stacker always
run --new-thread --use-stacker once --red-zone $((4 * 1024 * 1024))
