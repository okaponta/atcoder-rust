use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
    }
    let mut ans = vec![vec![0; n]; n];
    let d = vec![(0, 1), (1, 0), (0, !0), (!0, 0)];
    let mut di = 0;
    let mut x = 0;
    let mut y = 0;
    let mut cur = 1;
    while cur <= n * n {
        ans[x][y] = cur;
        let nx = x.wrapping_add(d[di].0);
        let ny = y.wrapping_add(d[di].1);
        if n <= nx || n <= ny || ans[nx][ny] != 0 {
            di = (di + 1) % 4;
            x = x.wrapping_add(d[di].0);
            y = y.wrapping_add(d[di].1);
        } else {
            x = nx;
            y = ny;
        }
        cur += 1;
    }
    for i in 0..n {
        println!(
            "{}",
            ans[i]
                .iter()
                .map(|i| if *i == n * n {
                    "T".to_string()
                } else {
                    i.to_string()
                })
                .join(" ")
        );
    }
}
