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
        d:usize,
        s:[Chars;h],
    }
    let mut hum = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                hum.push((i, j));
            }
        }
    }
    let n = hum.len();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..i {
            let mut cnt = 0;
            for ii in 0..h {
                for jj in 0..w {
                    if s[ii][jj] == '#' {
                        continue;
                    }
                    if dist(hum[i], (ii, jj)) <= d || dist(hum[j], (ii, jj)) <= d {
                        cnt += 1;
                    }
                }
            }
            ans = ans.max(cnt);
        }
    }
    println!("{}", ans);
}

fn dist(a: (usize, usize), b: (usize, usize)) -> usize {
    abs(a.0, b.0) + abs(a.1, b.1)
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
