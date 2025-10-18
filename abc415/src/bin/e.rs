#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[i64;w];h],
        p:[i64;h+w-1],
    }
    let mut lower = -1;
    let mut upper = 1 << 60;
    while 1 < upper - lower {
        let mid = (lower + upper) / 2;
        if ok(h, w, mid, &a, &p) {
            upper = mid;
        } else {
            lower = mid;
        }
    }
    println!("{}", upper);
}

fn ok(h: usize, w: usize, init: i64, a: &Vec<Vec<i64>>, p: &Vec<i64>) -> bool {
    let mut grid = vec![vec![-1 << 30; w]; h];
    grid[0][0] = init + a[0][0];
    for i in 0..h + w - 1 {
        let mut j = if i < h - 1 { i } else { h - 1 };
        let mut k = i - j;
        loop {
            if p[i] <= grid[j][k] {
                if j < h - 1 {
                    grid[j + 1][k] = grid[j + 1][k].max(grid[j][k] + a[j + 1][k] - p[i]);
                }
                if k < w - 1 {
                    grid[j][k + 1] = grid[j][k + 1].max(grid[j][k] + a[j][k + 1] - p[i]);
                }
            }
            if j == 0 {
                break;
            }
            j -= 1;
            k += 1;
            if k == w {
                break;
            }
        }
    }
    p[h + w - 2] <= grid[h - 1][w - 1]
}
