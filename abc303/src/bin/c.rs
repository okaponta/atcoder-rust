use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut h:usize,
        k:usize,
        s:Chars,
        xy:[(i64,i64);m],
    }
    let mut x = 0;
    let mut y = 0;
    let mut set = xy.into_iter().collect::<HashSet<_>>();
    for i in 0..n {
        if h == 0 {
            println!("No");
            return;
        }
        if s[i] == 'R' {
            x += 1;
        }
        if s[i] == 'L' {
            x -= 1;
        }
        if s[i] == 'U' {
            y += 1;
        }
        if s[i] == 'D' {
            y -= 1;
        }
        h -= 1;
        if h < k {
            if set.contains(&(x, y)) {
                h = k;
                set.remove(&(x, y));
            }
        }
    }
    println!("Yes");
}
