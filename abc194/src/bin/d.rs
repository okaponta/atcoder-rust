use proconio::input;

fn main() {
    input! {
        n:f64,
    }
    let mut ans = 0.0;
    for i in 1..n as i32 {
        ans += n / (n - i as f64);
    }
    println!("{}", ans);
}
