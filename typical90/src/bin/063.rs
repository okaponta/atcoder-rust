use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        p:[[usize;w];h],
    }
    let mut ans = 0;
    for i in 1usize..1 << h {
        let mut first = true;
        let mut dp = vec![0; w];
        for j in 0..h {
            if i >> j & 1 == 1 {
                if first {
                    for k in 0..w {
                        dp[k] = p[j][k];
                    }
                    first = false;
                } else {
                    for k in 0..w {
                        if dp[k] != p[j][k] {
                            dp[k] = 0;
                        }
                    }
                }
            }
        }
        let mut map = HashMap::new();
        for j in dp {
            if j != 0 {
                *map.entry(j).or_insert(0) += 1;
            }
        }
        let tmp = map.into_iter().map(|(_, v)| v).max().unwrap_or(0);
        ans = ans.max(tmp * i.count_ones())
    }
    println!("{}", ans);
}
