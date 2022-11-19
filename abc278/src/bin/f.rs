use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut start = HashMap::new();
    for i in 0..n {
        (*start.entry(s[i][0]).or_insert(vec![])).push(i);
    }
    let mut next = vec![vec![]; n];
    for i in 0..n {
        next[i] = start.get(&s[i][s[i].len() - 1]).unwrap_or(&vec![]).to_vec();
    }
    let mut dp = vec![vec![false; n]; 1 << n];
    for i in (1..1 << n).rev() {
        for j in 0..n {
            if i >> j & 1 == 0 {
                continue;
            }
            for k in 0..n {
                if i >> k & 1 == 0 {
                    if next[j].contains(&k) {
                        if !dp[i | 1 << k][k] {
                            dp[i][j] = true;
                        }
                    }
                }
            }
        }
    }
    for i in 0..n {
        dp[0][i] = !dp[1 << i][i];
    }
    println!(
        "{}",
        if dp[0].iter().any(|b| *b) {
            "First"
        } else {
            "Second"
        }
    )
}
