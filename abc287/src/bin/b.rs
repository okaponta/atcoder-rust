use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
        t:[String;m],
    }
    let mut set = HashSet::new();
    for i in 0..m {
        set.insert(&t[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        let suf = (3..6).into_iter().map(|j| s[i][j]).collect::<String>();
        if set.contains(&suf) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
