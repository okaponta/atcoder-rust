use proconio::input;

fn main() {
    input! {
        n:usize,
        t:[u128;n],
    }
    let ans = t.into_iter().fold(1, |s, i| lcm(s, i));
    println!("{}", ans);
}

fn gcd(a: u128, b: u128) -> u128 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}
