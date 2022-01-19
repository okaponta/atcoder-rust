use std::collections::VecDeque;

use im_rc::HashSet;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,
       ab:[(Usize1,Usize1);m],
    }
    let mut path = vec![vec![]; n];
    for (a, b) in ab {
        path[a].push(b);
    }
    let mut ans: usize = 0;
    let mut q = VecDeque::new();
    for i in 0..n {
        let mut set = HashSet::new();
        q.push_back(i);
        while let Some(next) = q.pop_front() {
            if set.contains(&next) {
                continue;
            }
            ans += 1;
            set.insert(next);
            for &j in &path[next] {
                if !set.contains(&j) {
                    q.push_back(j);
                }
            }
        }
    }
    println!("{}", ans);
}
