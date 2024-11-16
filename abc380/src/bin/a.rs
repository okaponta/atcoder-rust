#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        s:Chars,
    }
    let mut cnt = vec![0; 3];
    for c in s {
        if c == '1' {
            cnt[0] += 1;
        } else if c == '2' {
            cnt[1] += 1;
        } else if c == '3' {
            cnt[2] += 1;
        }
    }
    println!("{}", if cnt == vec![1, 2, 3] { "Yes" } else { "No" });
}
