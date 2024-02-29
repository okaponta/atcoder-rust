use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let first = vec![('S', 'S'), ('S', 'W'), ('W', 'S'), ('W', 'W')];
    for (one, two) in first {
        let mut ans = vec![one, two];
        for i in 1..n - 1 {
            ans.push(nx(ans[i], ans[i - 1], s[i]));
        }
        if nx(ans[n - 2], ans[n - 1], s[n - 1]) == ans[0] && nx(ans[n - 1], ans[0], s[0]) == ans[1]
        {
            println!("{}", ans.iter().join(""));
            return;
        }
    }
    println!("-1");
}

fn nx(now: char, prev: char, c: char) -> char {
    if now == 'S' {
        if c == 'o' {
            return prev;
        } else {
            return nt(prev);
        }
    } else {
        if c == 'o' {
            return nt(prev);
        } else {
            return prev;
        }
    }
}

fn nt(c: char) -> char {
    if c == 'S' {
        'W'
    } else {
        'S'
    }
}
