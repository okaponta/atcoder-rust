#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        s:Chars,
    }
    let mut pos = vec![];
    for i in 0..26 {
        let c = (b'A' + i) as char;
        for j in 0..26 {
            if s[j] == c {
                pos.push(j);
            }
        }
    }
    let mut ans = 0;
    for i in 0..25 {
        ans += abs(pos[i], pos[i + 1]);
    }
    println!("{}", ans);
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
