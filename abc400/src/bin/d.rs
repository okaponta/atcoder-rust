#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
        a:Usize1,
        b:Usize1,
        c:Usize1,
        d:Usize1,
    }
    let mut ans = vec![vec![1 << 30; w]; h];
    let mut q = VecDeque::new();
    q.push_back((a, b, 0));
    while let Some((i, j, c)) = q.pop_front() {
        if ans[i][j] <= c {
            continue;
        }
        ans[i][j] = c;
        for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if h <= ni || w <= nj || s[ni][nj] == '#' {
                continue;
            }
            q.push_front((ni, nj, c));
        }
        for (di, dj) in vec![
            (!0, 0),
            (0, 1),
            (0, !0),
            (1, 0),
            (!1, 0),
            (0, 2),
            (0, !1),
            (2, 0),
        ] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if h <= ni || w <= nj {
                continue;
            }
            q.push_back((ni, nj, c + 1));
        }
    }
    println!("{}", ans[c][d]);
}
