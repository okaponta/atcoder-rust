use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        uvw: [(Usize1,Usize1,usize);n-1],
    }
    let mut edges = vec![vec![]; n];
    for (u, v, w) in uvw {
        edges[u].push((v, w % 2));
        edges[v].push((u, w % 2));
    }
    let mut dist = vec![0; n];
    dfs(0, 0, 0, &edges, &mut dist);
    for ans in dist {
        println!("{}", ans % 2)
    }
}

fn dfs(cur: usize, prev: usize, d: usize, edges: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>) {
    for &(next, w) in &edges[cur] {
        if next == prev {
            continue;
        }
        dist[next] = d + w;
        dfs(next, cur, d + w, edges, dist);
    }
}
