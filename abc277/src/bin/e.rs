use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        uva:[(Usize1,Usize1,u8);m],
        s:[Usize1;k],
    }
    let mut edges = vec![vec![]; 2 * n];
    let switch = s.into_iter().collect::<HashSet<_>>();
    for (u, v, a) in uva {
        if a == 1 {
            edges[u].push(v);
            edges[v].push(u);
        } else {
            edges[u + n].push(v + n);
            edges[v + n].push(u + n);
        }
    }
    let mut d = vec![-1; 2 * n];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0));
    while let Some((cur, dist)) = q.pop_front() {
        if d[cur] != -1 {
            continue;
        }
        d[cur] = dist;
        for &next in &edges[cur] {
            if d[next] == -1 {
                q.push_back((next, dist + 1));
            }
        }
        if switch.contains(&(cur % n)) {
            q.push_back(((cur + n) % (2 * n), dist));
        }
    }
    let a1 = d[n - 1];
    let a2 = d[2 * n - 1];
    if a1.max(a2) == -1 {
        println!("-1");
        return;
    }
    if 0 <= a1.min(a2) {
        println!("{}", a1.min(a2));
        return;
    }
    println!("{}", a1.max(a2));
}
