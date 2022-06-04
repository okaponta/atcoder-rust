#!/bin/bash

# Usage: at ./abcXXX, execute 'source ../support.sh'
export CONTEST=$(basename $PWD)

alias join="git add .&&git commit -m 'join $CONTEST'"

source ~/repos/rust-kyopro-type-converter/convert.sh

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

function problem() {
  command open https://atcoder.jp/contests/$1/tasks/$1_a
  command open https://atcoder.jp/contests/$1/tasks/$1_b
  command open https://atcoder.jp/contests/$1/tasks/$1_c
  command open https://atcoder.jp/contests/$1/tasks/$1_d
  command open https://atcoder.jp/contests/$1/tasks/$1_e
  command open https://atcoder.jp/contests/$1/tasks/$1_f
}

function qiita() {
  command touch commentary.md
  command echo "[${CONTEST}](https://atcoder.jp/contests/${CONTEST})お疲れ様でした！" >>./commentary.md
  command echo "早速解説です！！(A-Fまで)" >>./commentary.md
  command echo "" >>./commentary.md
  command echo "質問、感想はこの記事のコメントでも、[twitter](https://twitter.com/okaponta_)でも、大歓迎です！:muscle:" >>./commentary.md
  command echo "" >>./commentary.md
  p_lower=(a b c d e f)
  p_upper=(A B C D E F)
  for i in $(seq 1 6); do
    command echo "## [問題${p_upper[i]}](https://atcoder.jp/contests/${CONTEST}/tasks/${CONTEST}_${p_lower[i]})" >>./commentary.md
    command echo "" >>./commentary.md
    command echo "### 考察" >>./commentary.md
    command echo "" >>./commentary.md
    command echo "### 解答" >>./commentary.md
    command echo "" >>./commentary.md
    command echo '```Rust' >>./commentary.md
    command echo '```' >>./commentary.md
    command echo "" >>./commentary.md
  done
}
