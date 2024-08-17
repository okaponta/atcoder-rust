#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        r:[usize;n],
    }
    let mut cur = vec![1; n];
    if cur.iter().sum::<usize>() % k == 0 {
        println!("{}", cur.iter().join(" "));
    }
    while cur != r {
        cur = next(cur, &r);
        if cur.iter().sum::<usize>() % k == 0 {
            println!("{}", cur.iter().join(" "));
        }
    }
}

fn next(mut a: Vec<usize>, r: &Vec<usize>) -> Vec<usize> {
    let mut i = a.len() - 1;
    while a[i] == r[i] {
        a[i] = 1;
        i -= 1;
    }
    a[i] += 1;
    a
}
