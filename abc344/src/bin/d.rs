use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        t:Chars,
        n:usize,
    }
    let mut check = vec![];
    let m = t.len();
    for i in 0..m {
        let mut set = HashSet::new();
        let mut tmp = vec![];
        set.insert(tmp.clone());
        for i in i..t.len() {
            tmp.push(t[i]);
            set.insert(tmp.clone());
        }
        check.push(set);
    }

    let mut dp = vec![1 << 30; m + 1];
    dp[0] = 0;
    for _ in 0..n {
        input! {a:usize, s:[Chars;a]}
        for i in (0..m).rev() {
            for j in 0..a {
                if check[i].contains(&s[j]) {
                    dp[i + s[j].len()] = dp[i + s[j].len()].min(dp[i] + 1);
                }
            }
        }
    }
    if dp[m] == 1 << 30 {
        println!("-1");
        return;
    }
    println!("{}", dp[m]);
}
