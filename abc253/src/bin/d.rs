use proconio::input;

fn main() {
    input! {
        n:i64,
        a:i64,
        b:i64,
    }
    let all = tousa_sum(n, 1);
    let asum = tousa_sum(n, a);
    let bsum = tousa_sum(n, b);
    let absum = tousa_sum(n, lcm(a, b));
    println!("{}", all - asum - bsum + absum);
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn tousa_sum(max: i64, c: i64) -> i64 {
    let n = max / c;
    (c + n * c) * n / 2
}
