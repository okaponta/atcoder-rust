use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        c:[Chars;h],
    }
    let mut start = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                start = (i, j);
            }
        }
    }
    let mut cand = vec![];
    for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
        let x = start.0.wrapping_add(dx);
        let y = start.1.wrapping_add(dy);
        if h <= x || w <= y {
            continue;
        } else if c[x][y] == '.' {
            cand.push((x, y));
        }
    }
    while 1 < cand.len() {
        let (x, y) = cand.pop().unwrap();
        let goal = cand.clone().into_iter().collect::<HashSet<_>>();
        let mut q = VecDeque::new();
        let mut visited = vec![vec![false; w]; h];
        q.push_back((x, y));
        while let Some((x, y)) = q.pop_front() {
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;
            if goal.contains(&(x, y)) {
                println!("Yes");
                return;
            }
            for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                let xi = x.wrapping_add(dx);
                let yi = y.wrapping_add(dy);
                if h <= xi || w <= yi {
                    continue;
                }
                if !visited[xi][yi] && c[xi][yi] == '.' {
                    q.push_back((xi, yi));
                }
            }
        }
    }
    println!("No");
}
