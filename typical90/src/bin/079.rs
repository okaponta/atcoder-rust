use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        mut a:[[i64;w];h],
        b:[[i64;w];h],
    }
    let mut ans = 0;
    for (i, j) in iproduct!(0..h - 1, 0..w - 1) {
        let diff = b[i][j] - a[i][j];
        for (k, l) in iproduct!(0..2, 0..2) {
            a[i + k][j + l] += diff;
        }
        ans += diff.abs();
    }
    for (i, j) in iproduct!(1..3, 1..3) {
        if a[h - i][w - j] != b[h - i][w - j] {
            println!("No");
            return;
        }
    }
    println!("Yes");
    println!("{ans}")
}
