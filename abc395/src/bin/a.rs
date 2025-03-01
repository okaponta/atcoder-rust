use proconio::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let b = (1..n).all(|i| a[i - 1] < a[i]);
    println!("{}", if b { "Yes" } else { "No" });
}
