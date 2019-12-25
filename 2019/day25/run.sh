#!/bin/bash
# Day 25
set -e

cd "$(dirname "$0")"
BASEDIR="$PWD"

function intcode() {
	(cd "${BASEDIR}"/../intcode; cargo run -q --release -- "$@")
}

if [[ "$1" == "--solve" ]]; then
	intcode -A "${BASEDIR}"/input.txt <<EOF
north
north
take monolith
north
take hypercube
south
south
east
east
take easter egg
east
south
take ornament
west
south
drop planetoid
drop candy cane
drop spool of cat6
drop fixed point
west
west
EOF
else
	intcode -A "${BASEDIR}"/input.txt
fi

