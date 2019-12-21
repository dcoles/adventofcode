#!/bin/bash
# Day 21
set -e

cd "$(dirname "$0")"
BASEDIR="$PWD"

function intcode() {
	(cd "${BASEDIR}"/../intcode; cargo run -q --release -- "$@")
}

echo 'Part 1'
echo '======'
intcode -A "${BASEDIR}"/input.txt < part1.txt
echo

echo 'Part 2'
echo '======'
intcode -A "${BASEDIR}"/input.txt < part2.txt
