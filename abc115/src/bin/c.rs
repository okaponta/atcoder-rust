use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut h:[usize;n],
    }
    h.sort();
    let mut ans = h[n - 1];
    for i in 0..=n - k {
        ans = ans.min(h[i + k - 1] - h[i]);
    }
    println!(
        "{}",
        (0..=n - k)
            .into_iter()
            .map(|i| h[i + k - 1] - h[i])
            .min()
            .unwrap()
    );
}
