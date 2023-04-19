use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut a:[i64;n],
    }
    a.insert(0, 0);
    a.push(0);
    let s = (0..=n)
        .into_iter()
        .fold(0, |s, i| s + (a[i + 1] - a[i]).abs());
    for i in 1..=n {
        let diff = (a[i] - a[i - 1]).abs() + (a[i + 1] - a[i]).abs() - (a[i + 1] - a[i - 1]).abs();
        println!("{}", s - diff);
    }
}
