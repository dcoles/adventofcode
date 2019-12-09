#!/bin/bash
set -e

cd "$(dirname "$0")"
BASEDIR="${PWD}"
PROGRAM="${BASEDIR}/input.txt"

function intcode() {
    cargo run -q --release -- "$@"
}

cd "${BASEDIR}"/../intcode

echo "Part 1: BOOST keycode is $(intcode "${PROGRAM}" <<< 1)"
echo "Part 2: Coordinates of distress signal are $(intcode "${PROGRAM}" <<< 2)"
