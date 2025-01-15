use proconio::*;

fn main() {
    input! {
        m:usize,
        d:usize,
    }
    println!("{}", if m % d == 0 { "YES" } else { "NO" });
}
