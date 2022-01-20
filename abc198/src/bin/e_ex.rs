use proconio::{input, marker::Usize1};

fn dfs(v: usize, p: usize, c: &[usize], g: &[Vec<usize>], cnt: &mut [u32], ans: &mut Vec<usize>) {
    let x = c[v];
    if cnt[x] == 0 {
        ans.push(v + 1);
    }
    cnt[x] += 1;
    for &u in g[v].iter() {
        if u != p {
            dfs(u, v, c, g, cnt, ans);
        }
    }
    cnt[x] -= 1;
}

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        e: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in e {
        g[a].push(b);
        g[b].push(a);
    }
    let mut cnt = vec![0; 100000];
    let mut ans = vec![];
    dfs(0, n, &c, &g, &mut cnt, &mut ans);
    ans.sort();
    for a in ans {
        println!("{}", a);
    }
}
