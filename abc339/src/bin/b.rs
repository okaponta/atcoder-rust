use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
    }
    let mut ans = vec![vec!['.'; w]; h];
    let mut x = 0;
    let mut y = 0;
    let mut i = 0;
    let d = vec![(!0, 0), (0, 1), (1, 0), (0, !0)];
    for _ in 0..n {
        if ans[x][y] == '.' {
            ans[x][y] = '#';
            i = (i + 1) % 4;
        } else {
            ans[x][y] = '.';
            i = (i + 3) % 4;
        }
        x = x.wrapping_add(d[i].0);
        y = y.wrapping_add(d[i].1);
        if x == h {
            x = 0;
        }
        if h < x {
            x = h - 1;
        }
        if y == w {
            y = 0;
        }
        if w < y {
            y = w - 1;
        }
    }
    for i in 0..h {
        println!("{}", ans[i].iter().join(""));
    }
}
