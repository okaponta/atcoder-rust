use std::{cmp::Reverse, collections::BinaryHeap};

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
        t:usize,
        s:[Chars;h],
    }
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i, j);
            }
            if s[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }
    let mut lower = 0;
    let mut upper = t;
    while 1 < upper - lower {
        let mid = (upper + lower) / 2;
        if is_ok(h, w, t, start, goal, &s, mid) {
            lower = mid;
        } else {
            upper = mid
        }
    }
    println!("{}", lower);
}

fn is_ok(
    h: usize,
    w: usize,
    t: usize,
    start: (usize, usize),
    goal: (usize, usize),
    s: &Vec<Vec<char>>,
    mid: usize,
) -> bool {
    let mut tmp = vec![vec![1 << 60; w]; h];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start.0, start.1)));
    while let Some(Reverse((c, i, j))) = heap.pop() {
        if tmp[i][j] <= c {
            continue;
        }
        tmp[i][j] = c;
        for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if h <= ni || w <= nj {
                continue;
            }
            if s[ni][nj] == '#' {
                heap.push(Reverse((c + mid, ni, nj)));
            } else {
                heap.push(Reverse((c + 1, ni, nj)));
            }
        }
    }
    tmp[goal.0][goal.1] <= t
}
