use proconio::input;

fn main() {
    input! {
        n:usize,
        t:usize,
        tn:[usize;n],
    }
    println!(
        "{}",
        (0..n - 1).fold(t, |s, i| s + (tn[i + 1] - tn[i]).min(t))
    );
}
