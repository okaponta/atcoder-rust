use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let (_, _, d) = tree_diameter(&g);
    println!("{}", d + 1);
}

fn tree_diameter(edges: &Vec<Vec<usize>>) -> (usize, usize, usize) {
    let l = tree_diameter_dfs(edges, 0, !0);
    let r = tree_diameter_dfs(edges, l.1, !0);
    (l.1, r.1, r.0)
}

// (距離, to)の最大値を返却する
fn tree_diameter_dfs(edges: &Vec<Vec<usize>>, cur: usize, parent: usize) -> (usize, usize) {
    let mut ret = (0, cur);
    for &to in &edges[cur] {
        if to == parent {
            continue;
        }
        let mut next = tree_diameter_dfs(edges, to, cur);
        next.0 += 1;
        ret = ret.max(next);
    }
    ret
}
