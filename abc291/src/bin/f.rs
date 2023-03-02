use itertools::Itertools;
use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
    }
    let mut g = vec![vec![]; n];
    let mut rev_g = vec![vec![]; n];
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '1' {
                g[i].push(i + j + 1);
                rev_g[i + j + 1].push(i);
            }
        }
    }
    let mut d = vec![INF; n];
    d[0] = 0;
    for i in 0..n {
        for &next in &g[i] {
            d[next] = d[next].min(d[i] + 1);
        }
    }
    let mut rev_d = vec![INF; n];
    rev_d[n - 1] = 0;
    for i in (0..n).rev() {
        for &next in &rev_g[i] {
            rev_d[next] = rev_d[next].min(rev_d[i] + 1);
        }
    }
    let mut ans = vec![INF; n - 2];
    for i in 1..n - 1 {
        for j in 1..m {
            if i < j {
                continue;
            }
            let bef = i - j;
            for &after in &g[bef] {
                if after <= i {
                    continue;
                }
                ans[i - 1] = ans[i - 1].min(d[bef] + rev_d[after] + 1);
            }
        }
    }
    println!(
        "{}",
        ans.into_iter()
            .map(|i| if i == INF { -1 } else { i as i64 })
            .join(" ")
    );
}
