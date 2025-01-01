use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        uv:[(Usize1,Usize1);n-1],
    }
    let mut d1 = vec![0; n];
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
        d1[u] += 1;
        d1[v] += 1;
    }
    let mut ans = n;
    for i in 0..n {
        let mut tmp = vec![];
        for &j in &g[i] {
            tmp.push(d1[j] - 1);
        }
        tmp.sort();
        let mut ansi = 0;
        let o = g[i].len();
        for j in 0..o {
            ansi = ansi.max(1 + (o - j) + tmp[j] * (o - j));
        }
        ans = ans.min(n - ansi);
    }
    println!("{}", ans);
}
