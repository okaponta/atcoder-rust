#!/bin/bash

# Usage: at ./abcXXX, execute 'source ../support.sh'
export CONTEST=$(basename $PWD)

source ~/repos/rust-kyopro-type-converter/convert.sh

# .vscode/launch.json is laid down by `acrust new` from
# .acrust/template/copy/, so this only has to commit it.
function join() {
  command git add .
  command git commit -m "join $CONTEST"
}

function test() {
  command acrust test $1
}

function run() {
  command acrust run $1
}

function submit() {
  command acrust submit $1
}

function copy() {
  command acrust copy $1
}

function commit() {
  command git add ./src/bin/${1}.rs
  command git commit -m "add ${CONTEST}_$1"
}

# No argument opens every problem in the package. Reads the task IDs from
# Cargo.toml, so joint contests (abc042 c is arc058_a) open correctly.
function problem() {
  command acrust open $1
}
