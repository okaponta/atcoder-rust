use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        x:[usize;n],
    }
    println!(
        "{}",
        (1..n).fold(0, |s, i| s + ((x[i] - x[i - 1]) * a).min(b))
    );
}
