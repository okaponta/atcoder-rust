use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut h:[usize;n],
    }
    h.sort();
    println!(
        "{}",
        (0..=n - k)
            .into_iter()
            .map(|i| h[i + k - 1] - h[i])
            .min()
            .unwrap()
    );
}
