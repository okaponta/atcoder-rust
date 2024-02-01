use proconio::input;

fn main() {
    input! {
        n:usize,
        k:i64,
        a:[i64;n],
        b:[i64;n],
    }
    let min = (0..n).fold(0, |s, i| s + (a[i] - b[i]).abs());
    println!(
        "{}",
        if min <= k && (k - min) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
