#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    let mut ans = 0;
    loop {
        a.sort();
        a.reverse();
        if a[1] == 0 {
            println!("{ans}");
            break;
        }
        a[0] -= 1;
        a[1] -= 1;
        ans += 1;
    }
}
