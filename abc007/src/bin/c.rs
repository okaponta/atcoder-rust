#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:(Usize1,Usize1),
        g:(Usize1,Usize1),
        c:[Chars;h],
    }
    let mut ans = vec![vec![1 << 30; w]; h];
    let mut q = VecDeque::new();
    q.push_back((s.0, s.1, 0));
    while let Some((i, j, d)) = q.pop_front() {
        if ans[i][j] <= d {
            continue;
        }
        ans[i][j] = d;
        for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if h <= ni || w <= nj || c[ni][nj] == '#' {
                continue;
            }
            q.push_back((ni, nj, d + 1));
        }
    }
    println!("{}", ans[g.0][g.1]);
}
