#!/bin/bash

# Usage at ./abcXXX, execute 'source ../support.sh'
export CONTEST=`basename $PWD`

alias join="git add .&&git commit -m 'join $CONTEST'"

function test() {
  command cargo compete test $1
}

function run() {
  command cargo run --bin ${CONTEST}-$1
}

function submit() {
  command cargo compete submit $1
}

function commit() {
  command git add ./src/bin/${1}.rs
  command git commit -m "add ${CONTEST}_$1"
}
