#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        t:usize,
        a:usize,
    }
    println!(
        "{}",
        if (n + 1) / 2 <= t || (n + 1) / 2 <= a {
            "Yes"
        } else {
            "No"
        }
    );
}
