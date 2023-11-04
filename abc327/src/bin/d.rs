use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[Usize1;m],
        b:[Usize1;m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        if a[i] == b[i] {
            println!("No");
            return;
        }
        g[a[i]].push(b[i]);
        g[b[i]].push(a[i]);
    }
    println!("{}", if is_bipartite(n, &g) { "Yes" } else { "No" });
}

fn is_bipartite(n: usize, g: &Vec<Vec<usize>>) -> bool {
    let mut color = vec![0; n];
    for i in 0..n {
        if color[i] == 0 {
            if !dfs(i, 1, &mut color, g) {
                return false;
            }
        }
    }
    true
}

fn dfs(cur: usize, col: i32, color: &mut Vec<i32>, edge: &Vec<Vec<usize>>) -> bool {
    color[cur] = col;
    for &next in &edge[cur] {
        if color[next] == col {
            return false;
        }
        if color[next] == 0 {
            if !dfs(next, -col, color, edge) {
                return false;
            }
        }
    }
    true
}
