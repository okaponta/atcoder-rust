#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        p:[Usize1;n],
        q:[Usize1;n],
    }
    let mut map = vec![0; n];
    for i in 0..n {
        map[q[i]] = i;
    }
    let mut ans = vec![];
    for i in 0..n {
        ans.push(q[p[map[i]]] + 1);
    }
    println!("{}", ans.iter().join(" "));
}
