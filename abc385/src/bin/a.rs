#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut a:[usize;3],
    }
    if a[0] == a[1] && a[1] == a[2] {
        println!("Yes");
        return;
    }
    a.sort();
    println!("{}", if a[0] + a[1] == a[2] { "Yes" } else { "No" })
}
