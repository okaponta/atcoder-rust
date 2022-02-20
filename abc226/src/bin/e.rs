use proconio::{input, marker::Usize1};

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
    }
    if n != m {
        println!("{}", 0);
        return;
    }

    let mut edge = vec![vec![]; n];
    for (u, v) in uv {
        edge[u].push(v);
        edge[v].push(u);
    }

    let mut ans = 1;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            visited[i] = true;
            let (v_count, e_count) = dfs(i, &edge, &mut visited);
            if v_count != e_count / 2 {
                println!("{}", 0);
                return;
            }
            ans = ans * 2 % MOD;
        }
    }

    println!("{}", ans);
}

fn dfs(cur: usize, edge: &[Vec<usize>], visited: &mut [bool]) -> (usize, usize) {
    let mut v_count = 1;
    let mut e_count = 0;

    for &next in &edge[cur] {
        e_count += 1;
        if !visited[next] {
            visited[next] = true;
            let (vr, er) = dfs(next, edge, visited);
            v_count += vr;
            e_count += er
        }
    }
    (v_count, e_count)
}
