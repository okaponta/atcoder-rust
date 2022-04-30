use proconio::input;

fn main() {
    input! {
        a:i64,b:i64
    }
    println!("{}", lcm(a, b));
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
