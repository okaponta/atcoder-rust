use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        t:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut ng = vec![HashSet::new(); n];
    for (a, b) in ab {
        ng[a].insert(b);
        ng[b].insert(a);
    }
    println!("{}", dfs(n, t, 0, &mut vec![], &ng));
}

fn dfs(
    n: usize,
    t: usize,
    cur: usize,
    pair: &mut Vec<Vec<usize>>,
    ng: &Vec<HashSet<usize>>,
) -> usize {
    if cur == n {
        return if pair.len() == t { 1 } else { 0 };
    }
    let mut ans = 0;
    for i in 0..pair.len() {
        // 追加可能か調べる
        if (0..pair[i].len())
            .into_iter()
            .all(|j| !ng[cur].contains(&pair[i][j]))
        {
            pair[i].push(cur);
            ans += dfs(n, t, cur + 1, pair, ng);
            pair[i].pop();
        }
    }
    if pair.len() < t {
        pair.push(vec![cur]);
        ans += dfs(n, t, cur + 1, pair, ng);
        pair.pop();
    }
    ans
}
