use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        uv:[(Usize1,Usize1);n-1],
    }
    let mut edge = vec![vec![]; n];
    for (u, v) in uv {
        edge[u].push(v);
        edge[v].push(u);
    }
    let mut ans = vec![(0, 0); n];
    dfs(&mut ans, 0, 0, &mut 1, &edge);
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}

fn dfs(
    ans: &mut Vec<(usize, usize)>,
    cur: usize,
    prev: usize,
    child: &mut usize,
    edges: &Vec<Vec<usize>>,
) {
    if cur != 0 && edges[cur].len() == 1 {
        // 終点
        ans[cur] = (*child, *child);
        *child += 1;
        return;
    }
    let mut left = 2 << 30;
    let mut right = 0;
    for &next in edges[cur].iter() {
        if next == prev {
            continue;
        }
        dfs(ans, next, cur, child, edges);
        left = left.min(ans[next].0);
        right = right.max(ans[next].1);
    }
    ans[cur] = (left, right);
}
