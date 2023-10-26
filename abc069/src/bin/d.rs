use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
        a:[usize;n],
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut count = 0;
    let mut ans = vec![vec![0; w]; h];
    for _ in 0..h * w {
        ans[j][i] = k + 1;
        if j % 2 == 0 {
            if i == w - 1 {
                j += 1;
            } else {
                i += 1;
            }
        } else {
            if i == 0 {
                j += 1;
            } else {
                i -= 1;
            }
        }
        count += 1;
        if a[k] == count {
            k += 1;
            count = 0;
        }
    }
    for i in 0..h {
        println!("{}", ans[i].iter().join(" "));
    }
}
