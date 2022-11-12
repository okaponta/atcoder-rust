use std::collections::{HashMap, HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n],
    }
    let mut ladders = HashMap::new();
    for (a, b) in ab {
        (*ladders.entry(a).or_insert(vec![])).push(b);
        (*ladders.entry(b).or_insert(vec![])).push(a);
    }
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut ans = 0;
    let mut visited = HashSet::new();
    while let Some(floor) = q.pop_front() {
        if visited.contains(&floor) {
            continue;
        }
        ans = ans.max(floor);
        visited.insert(floor);
        if !ladders.contains_key(&floor) {
            continue;
        }
        for &next in &ladders[&floor] {
            if visited.contains(&next) {
                continue;
            }
            q.push_back(next);
        }
    }
    println!("{}", ans + 1);
}
