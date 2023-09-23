use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        k:usize,
    }
    let mut q = VecDeque::new();
    for i in 1usize..10 {
        q.push_back(i);
    }
    let mut ans = vec![];
    while let Some(i) = q.pop_front() {
        ans.push(i);
        for j in 0..i % 10 {
            q.push_back(i * 10 + j);
        }
    }
    ans.sort();
    println!("{}", ans[k - 1]);
}
