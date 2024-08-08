#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut ans = vec![vec!['.'; w]; h];
    let d = vec![
        (!0, !0),
        (!0, 0),
        (!0, 1),
        (0, !0),
        (0, 0),
        (0, 1),
        (1, !0),
        (1, 0),
        (1, 1),
    ];
    for i in 0..h {
        for j in 0..w {
            let mut c = 0;
            for &(di, dj) in &d {
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if h <= ni || w <= nj || s[ni][nj] == '#' {
                    c += 1;
                }
            }
            if c == 9 {
                ans[i][j] = '#';
            }
        }
    }
    let mut t = vec![vec!['.'; w]; h];
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] == '#' {
                for &(di, dj) in &d {
                    let ni = i.wrapping_add(di);
                    let nj = j.wrapping_add(dj);
                    if h <= ni || w <= nj {
                        continue;
                    }
                    t[ni][nj] = '#';
                }
            }
        }
    }
    if s != t {
        println!("impossible");
        return;
    }
    println!("possible");
    for i in 0..h {
        println!("{}", ans[i].iter().join(""));
    }
}
