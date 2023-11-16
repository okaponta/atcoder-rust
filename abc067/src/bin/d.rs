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
    let mut fen = vec![0; n];
    let mut snu = vec![0; n];
    dfs(0, 0, 0, &mut fen, &g);
    dfs(n - 1, n - 1, 0, &mut snu, &g);
    let cnt = (0..n).into_iter().filter(|&i| fen[i] <= snu[i]).count();
    println!("{}", if n - cnt < cnt { "Fennec" } else { "Snuke" });
}

fn dfs(cur: usize, prev: usize, d: usize, dist: &mut Vec<usize>, g: &Vec<Vec<usize>>) {
    dist[cur] = d;
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        dfs(next, cur, d + 1, dist, g);
    }
}
