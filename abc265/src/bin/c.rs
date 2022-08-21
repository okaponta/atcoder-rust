use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        g:[Chars;h],
    }
    let mut set = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    loop {
        let c = g[x][y];
        if set.contains(&(x, y)) {
            println!("-1");
            return;
        }
        set.insert((x, y));
        if c == 'U' {
            if x == 0 {
                println!("{} {}", x + 1, y + 1);
                return;
            }
            x -= 1;
        }
        if c == 'D' {
            if x == h - 1 {
                println!("{} {}", x + 1, y + 1);
                return;
            }
            x += 1;
        }
        if c == 'R' {
            if y == w - 1 {
                println!("{} {}", x + 1, y + 1);
                return;
            }
            y += 1;
        }
        if c == 'L' {
            if y == 0 {
                println!("{} {}", x + 1, y + 1);
                return;
            }
            y -= 1;
        }
    }
}
