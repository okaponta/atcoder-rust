use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        s:Chars,
    }
    let mut mdp = vec![0; 3];
    let mut edp = HashMap::new();
    let mut ans = 0usize;
    for i in 0..n {
        if s[i] == 'M' {
            mdp[a[i]] += 1;
        } else if s[i] == 'E' {
            for j in 0..3 {
                *edp.entry((a[i].min(j), a[i].max(j))).or_insert(0) += mdp[j];
            }
        } else {
            for ((b, c), v) in edp.iter() {
                ans += points(a[i], *b, *c) * v;
            }
        }
    }
    println!("{}", ans);
}

fn points(a: usize, b: usize, c: usize) -> usize {
    for i in 0..3 {
        if a != i && b != i && c != i {
            return i;
        }
    }
    3
}
