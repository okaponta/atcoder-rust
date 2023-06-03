use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        d:i64,
        xy:[(i64,i64);n],
    }
    let mut affected = vec![false; n];
    affected[0] = true;
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(i) = q.pop_front() {
        for j in 0..n {
            if affected[j] {
                continue;
            }
            if dist(xy[i], xy[j]) <= d * d {
                affected[j] = true;
                q.push_back(j);
            }
        }
    }
    for i in 0..n {
        println!("{}", if affected[i] { "Yes" } else { "No" });
    }
}

fn dist(x: (i64, i64), y: (i64, i64)) -> i64 {
    (x.0 - y.0) * (x.0 - y.0) + (x.1 - y.1) * (x.1 - y.1)
}
