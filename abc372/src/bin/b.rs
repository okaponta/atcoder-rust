#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut m:usize,
    }
    let mut ans = vec![];
    for i in (0..=10).rev() {
        let tmp = 3usize.pow(i);
        while tmp <= m {
            ans.push(i);
            m -= tmp;
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
