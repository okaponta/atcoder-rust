use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,
       ab:[(Usize1,Usize1);m],
    }
    let mut road = vec![vec![]; n];
    for (a, b) in ab {
        road[a].push(b);
        road[b].push(a);
    }

    let mut ans = vec![1_000_000; n];
    let mut q = VecDeque::new();
    q.push_back(0);

    while let Some(prev) = q.pop_front() {
        for &next in road[prev].iter() {
            if ans[next] == 1_000_000 {
                ans[next] = prev;
                q.push_back(next);
            }
        }
    }

    println!("Yes");
    for i in 1..n {
        println!("{}", ans[i] + 1);
    }
}
