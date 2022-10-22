use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut dp = (-a[0], a[0]);
    for i in 1..n - 1 {
        let a = a[i];
        dp = ((dp.0 + a).max(dp.1 - a), (dp.0 - a).max(dp.1 + a));
    }
    println!("{}", (dp.0 - a[n - 1]).max(dp.1 + a[n - 1]));
}
