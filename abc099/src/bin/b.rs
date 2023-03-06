use proconio::input;

fn main() {
    input! {
        a:i64,
        b:i64,
    }
    println!("{}", (1..b - a).into_iter().fold(-a, |s, i| s + i));
}
