use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut cnt = vec![0; 5];
    for s in s {
        match s[0] {
            'M' => cnt[0] += 1,
            'A' => cnt[1] += 1,
            'R' => cnt[2] += 1,
            'C' => cnt[3] += 1,
            'H' => cnt[4] += 1,
            _ => (),
        }
    }
    let mut ans = 0usize;
    for (a, b, c) in (0..5).tuple_combinations() {
        ans += cnt[a] * cnt[b] * cnt[c];
    }
    println!("{}", ans);
}
