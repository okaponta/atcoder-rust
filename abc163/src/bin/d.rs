use proconio::input;

fn main() {
    input! {
        n:usize,k:usize,
    }
    println!(
        "{}",
        (k..=(n + 1))
            .into_iter()
            .fold(0, |s, i| (s + i * (n + 1 - i) + 1) % 1_000_000_007)
    );
}
