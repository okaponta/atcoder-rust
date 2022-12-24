use proconio::input;

fn main() {
    input! {
        n:usize,
        xx:i64,
        x:[i64;n],
    }
    println!("{}", x.into_iter().fold(0, |a, x| gcd(a, (x - xx).abs())));
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
