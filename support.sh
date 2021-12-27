#!/bin/bash

# Usage at ./abcXXX, execute 'source ../support.sh'
export CONTEST=`basename $PWD`
alias join="git add .&&git commit -m 'join $CONTEST'"
alias a="git add ./src/bin/a.rs&&git commit -m 'add ${CONTEST}_a'"
alias b="git add ./src/bin/b.rs&&git commit -m 'add ${CONTEST}_b'"
alias c="git add ./src/bin/c.rs&&git commit -m 'add ${CONTEST}_c'"
alias d="git add ./src/bin/d.rs&&git commit -m 'add ${CONTEST}_d'"
alias e="git add ./src/bin/e.rs&&git commit -m 'add ${CONTEST}_e'"
alias f="git add ./src/bin/f.rs&&git commit -m 'add ${CONTEST}_f'"
alias g="git add ./src/bin/g.rs&&git commit -m 'add ${CONTEST}_g'"
alias h="git add ./src/bin/h.rs&&git commit -m 'add ${CONTEST}_h'"
