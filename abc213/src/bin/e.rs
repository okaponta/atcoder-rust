use std::collections::VecDeque;

use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,w:usize,
        s:[Chars;h],
    }
    let hh = h as i32;
    let ww = w as i32;
    let mut is_road = vec![vec![false; w]; h];
    let mut ans = vec![vec![1 << 30; w]; h];
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == '.' {
            is_road[i][j] = true;
        }
    }

    let walk = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let punch = vec![
        (-2, -1),
        (-2, 0),
        (-2, 1),
        (-1, -2),
        (-1, -1),
        (-1, 1),
        (-1, 2),
        (0, -2),
        (0, 2),
        (1, -2),
        (1, -1),
        (1, 1),
        (1, 2),
        (2, -1),
        (2, 0),
        (2, 1),
    ];

    let mut q = VecDeque::new();
    q.push_back((0, 0, 0));
    while let Some((i, j, c)) = q.pop_front() {
        let ii = i as usize;
        let jj = j as usize;
        if ans[ii][jj] <= c {
            continue;
        }
        ans[ii][jj] = c;
        // となり
        for (x, y) in &walk {
            if is_available(i + x, j + y, hh, ww) {
                if is_road[(i + x) as usize][(j + y) as usize] {
                    q.push_front((i + x, j + y, c));
                }
            }
        }
        // パンチ
        for (x, y) in &punch {
            if is_available(i + x, j + y, hh, ww) {
                q.push_back((i + x, j + y, c + 1));
            }
        }
    }
    println!("{}", ans[h - 1][w - 1]);
}

fn is_available(i: i32, j: i32, h: i32, w: i32) -> bool {
    0 <= i && i < h && 0 <= j && j < w
}
