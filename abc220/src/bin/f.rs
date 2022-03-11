use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        uv :[(Usize1,Usize1);n-1],
    }
    let mut edge = vec![vec![]; n];
    for (u, v) in uv {
        edge[u].push(v);
        edge[v].push(u);
    }
    let mut dist = vec![-1; n];
    let mut size = vec![0; n];
    dfs(&mut dist, &mut size, 0, 0, &edge);

    let mut ans = vec![0; n];
    ans[0] = dist.iter().sum();
    dfs_ans(&mut ans, &size, 0, 0, n as i64, &edge);
    for a in ans {
        println!("{}", a);
    }
}

fn dfs_ans(
    ans: &mut Vec<i64>,
    size: &Vec<i64>,
    cur: usize,
    prev: usize,
    n: i64,
    edge: &Vec<Vec<usize>>,
) {
    if cur != 0 {
        ans[cur] = ans[prev] + n - 2 * size[cur];
    }
    for &next in edge[cur].iter() {
        if next == prev {
            continue;
        }
        dfs_ans(ans, size, next, cur, n, edge);
    }
}

fn dfs(dist: &mut Vec<i64>, size: &mut Vec<i64>, cur: usize, prev: usize, edge: &Vec<Vec<usize>>) {
    dist[cur] = dist[prev] + 1;
    let mut s = 1;
    for &next in edge[cur].iter() {
        if next == prev {
            continue;
        }
        dfs(dist, size, next, cur, edge);
        s += size[next];
    }
    size[cur] = s;
}
