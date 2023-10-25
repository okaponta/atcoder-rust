use proconio::input;

fn main() {
    input! {
        n:usize,
        _:i64,
        w:i64,
        a:[i64;n],
    }
    if n == 1 {
        println!("{}", (w - a[0]).abs());
    } else {
        println!("{}", (w - a[n - 1]).abs().max((a[n - 1] - a[n - 2]).abs()));
    }
}
