use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        r:usize,
        c:usize,
        k:usize,
        rcv:[(Usize1,Usize1,usize);k],
    }
    let mut mapv = vec![HashMap::new(); r];
    for (r, c, v) in rcv {
        mapv[r].insert(c, v);
    }
    let mut dp = vec![vec![0; 4]; c];
    for i in 0..r {
        let mut next = vec![vec![0; 4]; c];
        for j in 0..c {
            next[j][0] = *dp[j].iter().max().unwrap_or(&0);
        }
        for j in 0..c {
            let v = *mapv[i].get(&j).unwrap_or(&0);
            if j == 0 {
                if v != 0 {
                    next[0][1] = next[0][0] + v;
                }
                continue;
            }

            for k in 0..4 {
                next[j][k] = next[j][k].max(next[j - 1][k]);
            }
            if v == 0 {
                continue;
            }
            for k in (1..4).rev() {
                if next[j - 1][k - 1] != 0 {
                    next[j][k] = next[j][k].max(next[j - 1][k - 1] + v);
                }
                if k == 1 {
                    next[j][1] = next[j][1].max(next[j][0] + v);
                }
            }
        }
        dp = next;
    }
    println!("{}", dp[c - 1].iter().max().unwrap_or(&0))
}
