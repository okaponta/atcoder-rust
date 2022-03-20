use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut edge = vec![vec![]; n];
    for (u, v) in uv {
        edge[u].push(v);
        edge[v].push(u);
    }
    let mut ans = 0;
    let mut ans_set = HashSet::new();
    // setで管理せずに、let mut dist = vec![vec![INF; 1 << n]; n];とかで管理するとはやい
    let mut set = HashSet::new();
    ans_set.insert(0);
    let mut q = VecDeque::new();
    for i in 0..n {
        q.push_back((1 << i, i, 1));
        set.insert((1 << i, i));
        ans_set.insert(1 << i);
        ans += 1;
    }
    while ans_set.len() != 1 << n {
        let target = q.pop_front().unwrap();
        for &next_one in &edge[target.1] {
            let next_zero = target.0 ^ (1 << next_one);
            if set.contains(&(next_zero, next_one)) {
                continue;
            }
            let next_two = target.2 + 1;
            q.push_back((next_zero, next_one, next_two));
            set.insert((next_zero, next_one));
            if ans_set.contains(&next_zero) {
                continue;
            }
            ans_set.insert(next_zero);
            ans += next_two;
        }
    }
    println!("{}", ans);
}
