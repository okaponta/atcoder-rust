use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        ab:[(Usize1,Usize1);n-1],
        px:[(Usize1,usize);q],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut count = vec![0; n];
    let mut ans = vec![0; n];
    for (p, x) in px {
        count[p] += x;
    }
    dfs(0, 0, &count, &edges, &mut ans);
    println!("{}", ans.iter().join(" "));
}

fn dfs(cur: usize, prev: usize, count: &Vec<usize>, edges: &Vec<Vec<usize>>, ans: &mut Vec<usize>) {
    ans[cur] = ans[prev] + count[cur];
    for &next in &edges[cur] {
        if next == prev {
            continue;
        }
        dfs(next, cur, count, edges, ans);
    }
}
