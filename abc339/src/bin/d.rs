use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut s:[Chars;n],
    }
    let mut p = vec![];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                p.push((i, j));
                s[i][j] = '.';
            }
        }
    }
    let mut used = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((p[0], p[1], 0));
    used.insert((p[0], p[1]));
    while let Some((p1, p2, c)) = q.pop_front() {
        for d in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let np1 = mov(p1, d, &s, n);
            let np2 = mov(p2, d, &s, n);
            if np1 == np2 {
                println!("{}", c + 1);
                return;
            }
            if used.contains(&(np1, np2)) || used.contains(&(np2, np2)) {
                continue;
            }
            q.push_back((np1, np2, c + 1));
            used.insert((np1, np2));
        }
    }
    println!("-1");
}

fn mov(p: (usize, usize), d: (usize, usize), s: &Vec<Vec<char>>, n: usize) -> (usize, usize) {
    let nx = p.0.wrapping_add(d.0);
    let ny = p.1.wrapping_add(d.1);
    if n <= nx || n <= ny {
        return p;
    }
    if s[nx][ny] == '#' {
        return p;
    }
    (nx, ny)
}
