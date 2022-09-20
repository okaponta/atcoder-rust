use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        k:usize,
        s:[Chars;n],
    }
    let mut prev = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '.' {
                prev.insert(vec![(i, j)]);
            }
        }
    }
    for _ in 1..k {
        let mut next = HashSet::new();
        for pre in prev {
            let p = pre.clone();
            for (x, y) in pre {
                for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                    let nx = x.wrapping_add(dx);
                    let ny = y.wrapping_add(dy);
                    if n <= nx || n <= ny || s[nx][ny] == '#' || p.contains(&(nx, ny)) {
                        continue;
                    }
                    let mut nex = p.clone();
                    nex.push((nx, ny));
                    nex.sort();
                    next.insert(nex);
                }
            }
        }
        prev = next;
    }
    println!("{}", prev.len())
}
