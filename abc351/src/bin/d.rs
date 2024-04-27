use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut ans = 1;
    let is_neighbor = neigh(h, w, &s);
    let mut used = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if used[i][j] || s[i][j] == '#' || is_neighbor[i][j] {
                continue;
            }
            let mut count = 1;
            let mut q = VecDeque::new();
            q.push_back((i, j));
            used[i][j] = true;
            let mut set = HashSet::new();
            while let Some((i, j)) = q.pop_front() {
                for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                    let nx = i.wrapping_add(dx);
                    let ny = j.wrapping_add(dy);
                    if h <= nx || w <= ny {
                        continue;
                    }
                    if used[nx][ny] || set.contains(&(nx, ny)) {
                        continue;
                    }
                    if is_neighbor[nx][ny] && !set.contains(&(nx, ny)) {
                        count += 1;
                        set.insert((nx, ny));
                    } else {
                        if !used[nx][ny] {
                            q.push_back((nx, ny));
                            used[nx][ny] = true;
                            count += 1;
                        }
                    }
                }
            }
            ans = ans.max(count);
        }
    }
    println!("{}", ans);
}

fn neigh(h: usize, w: usize, s: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut res = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            let mut tmp = false;
            for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                let nx = i.wrapping_add(dx);
                let ny = j.wrapping_add(dy);
                if h <= nx || w <= ny {
                    continue;
                }
                if s[nx][ny] == '#' {
                    tmp = true;
                }
            }
            res[i][j] = tmp;
        }
    }
    res
}
