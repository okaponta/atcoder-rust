use std::collections::VecDeque;

use itertools::{iproduct, Itertools};
use num::Signed;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,w:usize,
        s:[Chars;h],
    }
    let mut is_road = vec![vec![false; w]; h];
    let mut ans = vec![vec![1 << 30; w]; h];
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == '.' {
            is_road[i][j] = true;
        }
    }

    let walk = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let punch = (-2..3)
        .cartesian_product(-2..3)
        .filter(|(i, j)| i.abs() + j.abs() != 4)
        .collect_vec();

    let mut q = VecDeque::new();
    q.push_back((0, 0, 0));
    while let Some((i, j, c)) = q.pop_front() {
        if ans[i][j] <= c {
            continue;
        }
        ans[i][j] = c;
        // となり
        for (x, y) in &walk {
            let next = ((i as i32 + x), (j as i32 + y));
            if is_available(next.0, next.1, h, w) {
                if is_road[next.0 as usize][next.1 as usize] {
                    q.push_front((next.0 as usize, next.1 as usize, c));
                }
            }
        }
        // パンチ
        for (x, y) in &punch {
            let next = ((i as i32 + x), (j as i32 + y));
            if is_available(next.0, next.1, h, w) {
                q.push_back((next.0 as usize, next.1 as usize, c + 1));
            }
        }
    }
    println!("{}", ans[h - 1][w - 1]);
}

fn is_available(i: i32, j: i32, h: usize, w: usize) -> bool {
    0 <= i && i < h as i32 && 0 <= j && j < w as i32
}
