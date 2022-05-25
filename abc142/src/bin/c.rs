use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    println!(
        "{}",
        (0..n)
            .sorted_by_key(|&i| a[i])
            .into_iter()
            .map(|i| i + 1)
            .join(" ")
    );
}
