use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let ans = (0..10)
        .cartesian_product(0..10)
        .into_iter()
        .map(|(a, b)| a * b)
        .any(|i| i == n);
    println!("{}", if ans { "Yes" } else { "No" });
}
