#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
    }
    let mut a = vec![];
    for i in 0..n {
        input! {ai: [Usize1; i+1]}
        a.push(ai);
    }
    let mut tmp = 0;
    for i in 0..n {
        if i < tmp {
            tmp = a[tmp][i];
        } else {
            tmp = a[i][tmp];
        }
    }
    println!("{}", tmp + 1);
}
