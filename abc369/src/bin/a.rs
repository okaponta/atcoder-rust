#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {a:i64,b:i64}
    let mut ans = 0;
    for i in -200..200 {
        let mut v = vec![a, b];
        v.push(i);
        v.sort();
        if v[2] - v[1] == v[1] - v[0] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
