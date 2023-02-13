use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
    }
    let set = a.into_iter().collect::<HashSet<_>>();
    let mut q = VecDeque::new();
    let mut ans = vec![];
    for i in 1..=n {
        q.push_front(i);
        if !set.contains(&i) {
            while let Some(j) = q.pop_front() {
                ans.push(j);
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}
