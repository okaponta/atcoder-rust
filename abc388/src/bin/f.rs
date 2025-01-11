use proconio::{marker::*, *};
use std::collections::HashSet;

fn main() {
    input! {
        n:Usize1,
        m:usize,
        a:usize,
        b:usize,
        lr:[(Usize1,usize);m],
    }
    if a == b {
        for &(l, r) in &lr {
            if l % a == 0 || (l / a) * a + a < r {
                println!("No");
                return;
            }
        }
        println!("{}", if n % a != 0 { "No" } else { "Yes" });
        return;
    }
    let mut dp = vec![false; b * b + 1];
    dp[0] = true;
    for i in 0..b * b {
        if !dp[i] {
            continue;
        }
        for j in a..=b {
            if i + j <= b * b {
                dp[i + j] = true;
            }
        }
    }
    let mut ybef = HashSet::new();
    ybef.insert(0);
    let mut yall = HashSet::new();
    for &(l, r) in &lr {
        let mut x = HashSet::new();
        for i in ybef {
            for j in (l.saturating_sub(b)).max(i)..l {
                if b * b < j - i || dp[j - i] {
                    x.insert(j);
                }
            }
        }
        let mut y = HashSet::new();
        for i in x {
            for j in a..=b {
                if r <= i + j {
                    y.insert(i + j);
                    yall.insert(i + j);
                }
            }
        }
        for i in 0..b {
            if yall.contains(&(r + i)) {
                y.insert(r + i);
            }
        }
        ybef = y;
    }
    for i in ybef {
        if n < i {
            continue;
        }
        if b * b < n - i || dp[n - i] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
