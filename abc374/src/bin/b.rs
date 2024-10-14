#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut s:Chars,
        mut t:Chars,
    }
    if s == t {
        println!("0");
        return;
    }
    if s.len() != t.len() {
        for _i in 0..200 {
            s.push('#');
            t.push('!');
        }
    }
    for i in 0..s.len().min(t.len()) {
        if s[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
    }
}
