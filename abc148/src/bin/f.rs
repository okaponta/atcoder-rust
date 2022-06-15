use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        u:Usize1,v:Usize1,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut taka = vec![1 << 60; n];
    taka[u] = 0;
    dfs(u, u, &edges, &mut taka);
    let mut aoki = vec![1 << 60; n];
    aoki[v] = 0;
    dfs(v, v, &edges, &mut aoki);
    let mut ans = 0;
    for i in 0..n {
        if taka[i] < aoki[i] {
            ans = ans.max(aoki[i] - 1);
        }
    }
    println!("{}", ans);
}

fn dfs(prev: usize, cur: usize, edges: &Vec<Vec<usize>>, d: &mut Vec<usize>) {
    for &next in &edges[cur] {
        if next == prev {
            continue;
        }
        d[next] = d[cur] + 1;
        dfs(cur, next, edges, d);
    }
}
