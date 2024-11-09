#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut s:Chars,
    }
    let mut ans = 0;
    for i in 0..n {
        s.push('X');
        if (i..i + k).all(|j| s[j] == 'O') {
            ans += 1;
            for j in 0..k {
                s[i + j] = 'X';
            }
        }
    }
    println!("{}", ans);
}
