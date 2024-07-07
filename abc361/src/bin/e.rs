use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        abc:[(Usize1,Usize1,usize);n-1],
    }
    let mut g = vec![vec![]; n];
    let mut ans = 0;
    for (a, b, c) in abc {
        ans += c * 2;
        g[a].push((b, c));
        g[b].push((a, c));
    }
    println!("{}", ans - tree_diameter_cost(&g).2);
}

fn tree_diameter_cost(edges: &Vec<Vec<(usize, usize)>>) -> (usize, usize, usize) {
    let l = tree_diameter_dfs_cost(edges, 0, !0);
    let r = tree_diameter_dfs_cost(edges, l.1, !0);
    (l.1, r.1, r.0)
}

fn tree_diameter_dfs_cost(
    edges: &Vec<Vec<(usize, usize)>>,
    cur: usize,
    parent: usize,
) -> (usize, usize) {
    let mut ret = (0, cur);
    for &(to, cost) in &edges[cur] {
        if to == parent {
            continue;
        }
        let mut next = tree_diameter_dfs_cost(edges, to, cur);
        next.0 += cost;
        ret = ret.max(next);
    }
    ret
}
