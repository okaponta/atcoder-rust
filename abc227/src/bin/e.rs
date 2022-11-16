use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
       s:Chars,
       k:usize,
    }
    let mut k_idx = vec![];
    let mut e_idx = vec![];
    let mut y_idx = vec![];
    for i in 0..s.len() {
        match s[i] {
            'K' => k_idx.push(i),
            'E' => e_idx.push(i),
            _ => y_idx.push(i),
        }
    }
    let s_num = s.len();
    let k_num = k_idx.len();
    let e_num = e_idx.len();
    let y_num = y_idx.len();
    // dp[x][e][y]..Eをe個、Yをy個含み、x回の操作で一致させるのに必要な個数
    let mut dp = vec![vec![vec![0i64; y_num + 1]; e_num + 1]; 1];
    dp[0][0][0] = 1;
    for i in 0..s_num {
        let mut next = vec![vec![vec![0; y_num + 1]; e_num + 1]; (i + 1) * s_num];
        for x in 0..dp.len() {
            for e in 0..=e_num {
                for y in 0..=y_num {
                    let k = i.saturating_sub(e + y);
                    if k_num < k {
                        continue;
                    }
                    let mut indexes = vec![];
                    for i in 0..k {
                        indexes.push(k_idx[i]);
                    }
                    for i in 0..e {
                        indexes.push(e_idx[i]);
                    }
                    for i in 0..y {
                        indexes.push(y_idx[i]);
                    }
                    indexes.sort();
                    // k
                    if k < k_num {
                        let inv = k_idx[k] - indexes.lower_bound(&k_idx[k]);
                        next[x + inv][e][y] += dp[x][e][y];
                    }
                    // e
                    if e < e_num {
                        let inv = e_idx[e] - indexes.lower_bound(&e_idx[e]);
                        next[x + inv][e + 1][y] += dp[x][e][y];
                    }
                    // y
                    if y < y_num {
                        let inv = y_idx[y] - indexes.lower_bound(&y_idx[y]);
                        next[x + inv][e][y + 1] += dp[x][e][y];
                    }
                }
            }
        }
        dp = next;
    }
    let mut ans = 0;
    for i in 0..dp.len() {
        if i <= k {
            ans += dp[i][e_num][y_num];
        }
    }
    println!("{}", ans);
}
