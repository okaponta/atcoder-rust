use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
    }
    let mut visited = vec![vec![false; m]; n];
    visited[1][1] = true;
    let mut set = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((1usize, 1usize));
    while let Some((i, j)) = q.pop_front() {
        if set.contains(&(i, j)) {
            continue;
        }
        set.insert((i, j));
        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let mut tmpi = i;
            let mut tmpj = j;
            let mut nexti = tmpi.wrapping_add(dx);
            let mut nextj = tmpj.wrapping_add(dy);
            while s[nexti][nextj] == '.' {
                tmpi = nexti;
                tmpj = nextj;
                visited[tmpi][tmpj] = true;
                nexti = tmpi.wrapping_add(dx);
                nextj = tmpj.wrapping_add(dy);
            }
            if set.contains(&(tmpi, tmpj)) {
                continue;
            }
            q.push_back((tmpi, tmpj));
        }
    }
    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if visited[i][j] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
