use proconio::*;
fn main() {
    input! {n:usize,k:usize,a:[usize;n]}
    println!(
        "{}",
        (0..n)
            .into_iter()
            .map(|i| a[i] * (i + 1).min(n - i).min(n + 1 - k).min(k))
            .sum::<usize>()
    )
}
