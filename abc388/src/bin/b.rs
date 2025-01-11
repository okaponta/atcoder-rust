#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        d:usize,
        tl:[(usize,usize);n],
    }
    for i in 1..=d {
        let mut ans = 0;
        for &(t, l) in &tl {
            ans = ans.max(t * (l + i));
        }
        println!("{}", ans);
    }
}
