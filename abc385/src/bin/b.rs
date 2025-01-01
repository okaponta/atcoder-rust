use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        h:usize,
        w:usize,
        mut x:Usize1,
        mut y:Usize1,
        s:[Chars;h],
        t:Chars,
    }
    let mut set = HashSet::new();
    if s[x][y] == '@' {
        set.insert((x, y));
    }
    for c in t {
        let d = match c {
            'U' => (!0, 0),
            'D' => (1, 0),
            'L' => (0, !0),
            'R' => (0, 1),
            _ => panic!(),
        };
        let nx = x.wrapping_add(d.0);
        let ny = y.wrapping_add(d.1);
        if h <= nx || w <= ny || s[nx][ny] == '#' {
            continue;
        }
        x = nx;
        y = ny;
        if s[x][y] == '@' {
            set.insert((x, y));
        }
    }
    println!("{} {} {}", x + 1, y + 1, set.len());
}
