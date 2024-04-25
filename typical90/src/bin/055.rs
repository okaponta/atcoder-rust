use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        p:usize,
        q:usize,
        a:[usize;n],
    }
    println!(
        "{}",
        (0..n)
            .combinations(5)
            .filter(|v| v.into_iter().fold(1, |pr, i| (pr * a[*i]) % p) == q)
            .count()
    );
}
