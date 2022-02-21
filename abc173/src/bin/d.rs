use proconio::input;

fn main() {
    input! {
       n:usize,
       mut a:[usize;n],
    }
    a.sort();
    println!(
        "{}",
        (1..n).into_iter().fold(0, |s, i| s + a[n - 1 - i / 2])
    );
}
