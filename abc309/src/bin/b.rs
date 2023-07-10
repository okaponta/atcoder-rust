use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[Chars;n],
    }
    let mut b = a.clone();
    for i in 0..n - 1 {
        b[0][i + 1] = a[0][i];
        b[i + 1][n - 1] = a[i][n - 1];
        b[n - 1][n - i - 2] = a[n - 1][n - i - 1];
        b[n - i - 2][0] = a[n - i - 1][0];
    }
    for i in 0..n {
        for j in 0..n {
            print!("{}", b[i][j]);
        }
        println!();
    }
}
