use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
       n:usize,
       ab: [(usize,usize);n]
    }
    let mut ans = 100000;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                ans = min(ans, ab[i].0 + ab[i].1);
            } else {
                ans = min(ans, max(ab[i].0, ab[j].1));
            }
        }
    }
    println!("{}", ans);
}
