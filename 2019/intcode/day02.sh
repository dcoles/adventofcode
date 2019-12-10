#!/bin/bash
# Advent of Code Day 7
set -e

cd "$(dirname "$0")"
BASEDIR="${PWD}"
PROG="${BASEDIR}/../day02/input.txt"

function build() {
  (cd "${BASEDIR}/../intcode"; cargo build --release)
}

function prog() {
  echo "$(cut -d ',' -f 1 "${PROG}"),$1,$2,$(cut -d ',' -f 4- "${PROG}")"
}

function run() {
  echo "$(target/release/intcode --dump <(prog "$1" "$2") 2>&1 | grep -F 00000000 | awk '{print $2}')"
}

function part1() {
  echo "Part 1: Position 0 = $(run 12 2)"
}

function part2() {
  echo This might take a while...
  for noun in $(seq 99); do
    echo -n .
    for verb in $(seq 99); do
      local value
      value="$(run "${noun}" "${verb}")"
      if [[ "${value}" -eq 19690720 ]]; then
        echo
        echo "Part 2: Inputs ${noun}, ${verb} give ${value} (answer: ${noun}${verb})"
        return
      fi
    done
  done

  wait
}

build
part1
part2
