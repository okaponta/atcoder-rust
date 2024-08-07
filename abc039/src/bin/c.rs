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
    let t = "WBWBWWBWBWBWWBWBWWBWBWBWWBWBWWBWBWBW"
        .chars()
        .collect::<Vec<_>>();
    let w = vec![0, 2, 4, 5, 7, 9, 11];
    let ans = vec!["Do", "Re", "Mi", "Fa", "So", "La", "Si"];
    for i in 0..7 {
        if (0..20).all(|j| s[j] == t[w[i] + j]) {
            println!("{}", ans[i]);
            return;
        }
    }
}
