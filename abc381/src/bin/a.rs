#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mid = n / 2;
    let ans = n % 2 == 1
        && s[mid] == '/'
        && (0..mid).all(|i| s[i] == '1')
        && (mid + 1..n).all(|i| s[i] == '2');
    println!("{}", if ans { "Yes" } else { "No" });
}
