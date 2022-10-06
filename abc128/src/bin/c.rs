use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut switches = vec![];
    for _ in 0..m {
        input! {k:usize,s:[Usize1;k]}
        switches.push(s.into_iter().collect::<HashSet<_>>());
    }
    input! {p:[usize;m]}
    let mut ans = 0;
    for i in 0..1 << n {
        let mut count = vec![0; m];
        for j in 0..n {
            if i >> j & 1 == 1 {
                for k in 0..m {
                    if switches[k].contains(&j) {
                        count[k] += 1;
                    }
                }
            }
        }
        if (0..m).into_iter().all(|k| count[k] % 2 == p[k]) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
