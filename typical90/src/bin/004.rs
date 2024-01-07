use itertools::{iproduct, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[usize;w];h],
    }
    let mut row = vec![0; h];
    let mut col = vec![0; w];
    for (i, j) in iproduct!(0..h, 0..w) {
        row[i] += a[i][j];
        col[j] += a[i][j];
    }
    let mut ans = vec![vec![0; w]; h];
    for (i, j) in iproduct!(0..h, 0..w) {
        ans[i][j] = row[i] + col[j] - a[i][j];
    }
    for i in 0..h {
        println!("{}", ans[i].iter().join(" "));
    }
}
