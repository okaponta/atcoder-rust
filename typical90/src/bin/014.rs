use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i64;n],
        mut b:[i64;n],
    }
    a.sort();
    b.sort();
    println!("{}", (0..n).fold(0, |s, i| s + (a[i] - b[i]).abs()));
}
