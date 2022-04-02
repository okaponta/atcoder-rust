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

function problem(){
  command open https://atcoder.jp/contests/$1/tasks/$1_a
  command open https://atcoder.jp/contests/$1/tasks/$1_b
  command open https://atcoder.jp/contests/$1/tasks/$1_c
  command open https://atcoder.jp/contests/$1/tasks/$1_d
  command open https://atcoder.jp/contests/$1/tasks/$1_e
  command open https://atcoder.jp/contests/$1/tasks/$1_f
}
