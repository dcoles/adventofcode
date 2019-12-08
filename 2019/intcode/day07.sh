#!/bin/bash
# Advent of Code Day 7
set -e

for AMP in target/{release,debug}/intcode{,.exe}; do
  if [[ -e "${AMP}" ]]; then
    break
  fi
done
PROG=../day07/input.txt

# Generate all permutations of $1
function permutations() {
  PERMUTATIONS=()
  while read -r line; do
    PERMUTATIONS+=("$line")
  done < <(python3 << EOF
import itertools
for perm in itertools.permutations([$1]):
    print(' '.join(str(x) for x in perm))
EOF
  )
}

function part1() {
  local max=0
  local phases=''
  local perm
  permutations "0,1,2,3,4"
  for perm in "${PERMUTATIONS[@]}"; do
    read -r -a p <<< "${perm}"

    local signal="$( (echo "${p[0]}"; echo 0) | $AMP "${PROG}" | (echo "${p[1]}"; cat) | $AMP "${PROG}" | (echo "${p[2]}"; cat) | $AMP "${PROG}" | (echo "${p[3]}"; cat) | $AMP "${PROG}" | (echo "${p[4]}"; cat) | $AMP "${PROG}" )"

    if [[ $signal -gt $max ]]; then
      max="${signal}"
      phases="${p[*]}"
    fi
  done

  echo "Part 1: Highest signal is ${max} (${phases})"
}

function part2() {
  local max=0
  local phases=''
  local perm
  permutations "5,6,7,8,9"
  for perm in "${PERMUTATIONS[@]}"; do
    read -r -a p <<< "${perm}"

    # Pipe to allow feedback of output to input
    pipe="$(mktemp -u)"
    mkfifo "${pipe}"
    exec 3<>"${pipe}"
    unlink "${pipe}"

    # Add first amp's phase and initial input
    echo "${p[0]}" >&3
    echo 0 >&3

    local signal
    signal="$( $AMP "${PROG}" <&3 | (echo "${p[1]}"; cat) | $AMP "${PROG}" | (echo "${p[2]}"; cat) | $AMP "${PROG}" | (echo "${p[3]}"; cat) | $AMP "${PROG}" | (echo "${p[4]}"; cat) | $AMP "${PROG}" | tee /dev/fd/3 | tail -n 1 )"

    if [[ "${signal}" -gt "${max}" ]]; then
      max="${signal}"
      phases="${p[*]}"
    fi
  done

  echo "Part 2: Highest signal is ${max} (${phases})"
}

part1
part2
