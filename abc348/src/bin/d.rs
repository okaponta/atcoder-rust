use std::{
    collections::{HashSet, VecDeque},
    vec,
};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[Chars;h],
        n:usize,
        rce:[(Usize1,Usize1,usize);n],
    }
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                start = (i, j);
            }
            if a[i][j] == 'T' {
                goal = (i, j);
            }
        }
    }
    let mut med = vec![vec![0; w]; h];
    for &(r, c, e) in &rce {
        med[r][c] = e;
    }
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(start);
    visited.insert(start);
    while let Some((i, j)) = q.pop_front() {
        let set = ok(i, j, med[i][j], h, w, &a);
        if set.contains(&goal) {
            println!("Yes");
            return;
        }
        for &(r, c, _e) in &rce {
            if visited.contains(&(r, c)) {
                continue;
            }
            if set.contains(&(r, c)) {
                q.push_back((r, c));
                visited.insert((r, c));
            }
        }
    }
    println!("No");
}

fn ok(
    x: usize,
    y: usize,
    d: usize,
    h: usize,
    w: usize,
    a: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    res.insert((x, y));
    let mut q = VecDeque::new();
    q.push_back((x, y, d));
    while let Some((x, y, d)) = q.pop_front() {
        if d == 0 {
            continue;
        }
        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let xi = x.wrapping_add(dx);
            let yi = y.wrapping_add(dy);
            if h <= xi || w <= yi {
                continue;
            }
            // ここは移動可能
            if res.contains(&(xi, yi)) || a[xi][yi] == '#' {
                continue;
            }
            q.push_back((xi, yi, d - 1));
            res.insert((xi, yi));
        }
    }
    res
}
