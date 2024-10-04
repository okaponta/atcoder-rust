#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut ans = vec![vec!['o'; n]; n];
    for i in 0..n {
        for j in 0..n {
            ans[j][n - 1 - i] = s[i][j];
        }
    }
    for i in 0..n {
        println!("{}", ans[i].iter().join(""));
    }
}
