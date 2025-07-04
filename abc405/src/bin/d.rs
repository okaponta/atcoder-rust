#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        h:usize,
        w:usize,
        mut s:[Chars;h],
    }
    let mut q = VecDeque::new();
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == 'E' {
            q.push_back((i, j));
        }
    }
    while let Some((i, j)) = q.pop_front() {
        for (di, dj, c) in vec![(!0, 0, 'v'), (0, 1, '<'), (0, !0, '>'), (1, 0, '^')] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if h <= ni || w <= nj {
                continue;
            }
            if s[ni][nj] == '.' {
                s[ni][nj] = c;
                q.push_back((ni, nj));
            }
        }
    }
    for s in s {
        println!("{}", s.iter().collect::<String>());
    }
}
