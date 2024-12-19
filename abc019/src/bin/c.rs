use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut set = HashSet::new();
    for mut a in a {
        while a % 2 & 1 == 0 {
            a >>= 1;
        }
        set.insert(a);
    }
    println!("{}", set.len());
}
