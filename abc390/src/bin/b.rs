use proconio::*;

fn main() {
    input! {n:usize,a:[usize;n]}
    let b = a.windows(3).all(|v| v[1] * v[1] == v[0] * v[2]);
    println!("{}", if b { "Yes" } else { "No" });
}
