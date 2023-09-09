use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut rev_g = vec![HashSet::new(); n];
    for (u, v) in uv {
        rev_g[u].insert(v);
        rev_g[v].insert(u);
    }

    let mut unvisited = (1..n).into_iter().collect::<HashSet<_>>();
    let mut dp = vec![0; n];
    dp[0] = 1;
    let mut q = VecDeque::from([(0, 1)]);

    let mut sum = vec![1];
    let mut edges = vec![HashSet::from([0])];

    let mut prev_d = 0;
    while let Some((cur, d)) = q.pop_front() {
        if prev_d < d {
            sum.push(0);
            edges.push(HashSet::new());
            prev_d = d;
        }
        let mut tmp = sum[d - 1];
        for &ng in &rev_g[cur] {
            if edges[d - 1].contains(&ng) {
                tmp = (MOD + tmp - dp[ng]) % MOD;
            }
        }
        if cur == n - 1 {
            println!("{}", tmp);
            return;
        }
        dp[cur] = tmp;
        sum[d] = (sum[d] + tmp) % MOD;
        edges[d].insert(cur);

        let mut visited = vec![];
        for &next in &unvisited {
            if !rev_g[cur].contains(&next) {
                visited.push(next);
                q.push_back((next, d + 1));
            }
        }
        for u in visited {
            unvisited.remove(&u);
        }
    }
    println!("-1");
}
