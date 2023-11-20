use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if i % 2 == 0 {
            q.push_back(a[i]);
        } else {
            q.push_front(a[i]);
        }
    }
    let mut ans = q.into_iter().collect::<Vec<_>>();
    if n % 2 == 1 {
        ans.reverse();
    }
    println!("{}", ans.iter().join(" "));
}
