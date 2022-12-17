use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
    }
    let mut ans = 0;
    for (a, b) in (0..n).into_iter().tuple_combinations() {
        let mut count = 0;
        for i in 0..m {
            if s[a][i] == 'x' && s[b][i] == 'x' {
                continue;
            }
            count += 1;
        }
        if count == m {
            ans += 1;
        }
    }
    println!("{}", ans);
}
