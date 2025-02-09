#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
    }
    let mut ans = vec![false; n + 1];
    for a in a {
        ans[a] = true;
    }
    let count = ans.iter().filter(|&&x| !x).count();
    println!("{}", count - 1);
    for i in 1..=n {
        if !ans[i] {
            print!("{} ", i);
        }
    }
    println!();
}
